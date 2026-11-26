use super::models::{Dashboard, Problem, ProblemDetail, ProblemInput, ProblemQuery};
use crate::core::{
    error::{AppError, AppResult},
    workspace::{ProblemManifest, source_files},
};
use rusqlite::{Connection, OptionalExtension, Row, params, params_from_iter};
use std::path::Path;

const SELECT: &str = "SELECT id, identifier, title, description, platform, url, difficulty, tags, status, languages, workspace_path, notes, created_at, updated_at, last_opened_at FROM problems";

pub fn create(conn: &Connection, input: ProblemInput, workspace_path: &str) -> AppResult<Problem> {
    available(conn, &input.identifier, None)?;
    conn.execute("INSERT INTO problems (identifier,title,description,platform,url,difficulty,tags,status,languages,workspace_path,notes) VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9,?10,?11)", params![input.identifier,input.title,input.description,input.platform,input.url,input.difficulty,serde_json::to_string(&input.tags)?,input.status,serde_json::to_string(&input.languages)?,workspace_path,input.notes])?;
    get(conn, conn.last_insert_rowid())
}
pub fn validate_new(conn: &Connection, input: &ProblemInput) -> AppResult<()> {
    available(conn, &input.identifier, None)
}
pub fn get(conn: &Connection, id: i64) -> AppResult<Problem> {
    conn.query_row(&format!("{SELECT} WHERE id=?1"), [id], from_row)
        .optional()?
        .ok_or_else(|| AppError::NotFound("Problem was not found".into()))
}
pub fn detail(conn: &Connection, id: i64) -> AppResult<ProblemDetail> {
    let problem = get(conn, id)?;
    Ok(ProblemDetail {
        source_files: source_files(Path::new(&problem.workspace_path), &problem.languages),
        problem,
    })
}
pub fn update(conn: &Connection, id: i64, input: ProblemInput) -> AppResult<Problem> {
    get(conn, id)?;
    available(conn, &input.identifier, Some(id))?;
    conn.execute("UPDATE problems SET identifier=?1,title=?2,description=?3,platform=?4,url=?5,difficulty=?6,tags=?7,status=?8,languages=?9,notes=?10,updated_at=strftime('%Y-%m-%dT%H:%M:%fZ','now') WHERE id=?11", params![input.identifier,input.title,input.description,input.platform,input.url,input.difficulty,serde_json::to_string(&input.tags)?,input.status,serde_json::to_string(&input.languages)?,input.notes,id])?;
    get(conn, id)
}
pub fn delete(conn: &Connection, id: i64) -> AppResult<()> {
    if conn.execute("DELETE FROM problems WHERE id=?1", [id])? == 0 {
        Err(AppError::NotFound("Problem was not found".into()))
    } else {
        Ok(())
    }
}
pub fn list(conn: &Connection, query: ProblemQuery) -> AppResult<Vec<Problem>> {
    let q = query.with_defaults();
    let mut clauses = vec![];
    let mut values = vec![];
    if !q.search.trim().is_empty() {
        clauses.push("(title LIKE ? OR identifier LIKE ? OR platform LIKE ?)".into());
        let v = format!("%{}%", q.search.trim());
        values.extend([v.clone(), v.clone(), v]);
    }
    for (value, column) in [
        (q.status.trim(), "status"),
        (q.difficulty.trim(), "difficulty"),
        (q.platform.trim(), "platform"),
    ] {
        if !value.is_empty() {
            clauses.push(format!("{column}=?"));
            values.push(value.into());
        }
    }
    if !q.language.trim().is_empty() {
        clauses.push("languages LIKE ?".into());
        values.push(format!("%\"{}\"%", q.language.trim()));
    }
    if !q.tag.trim().is_empty() {
        clauses.push("tags LIKE ?".into());
        values.push(format!("%\"{}\"%", q.tag.trim()));
    }
    let where_sql = if clauses.is_empty() {
        String::new()
    } else {
        format!(" WHERE {}", clauses.join(" AND "))
    };
    let sort = match q.sort.as_str() {
        "title" => "title COLLATE NOCASE",
        "created" => "created_at",
        "difficulty" => {
            "CASE difficulty WHEN 'easy' THEN 1 WHEN 'medium' THEN 2 WHEN 'hard' THEN 3 ELSE 4 END"
        }
        "status" => {
            "CASE status WHEN 'not_started' THEN 1 WHEN 'in_progress' THEN 2 WHEN 'review' THEN 3 ELSE 4 END"
        }
        _ => "updated_at",
    };
    let direction = if q.sort_direction.eq_ignore_ascii_case("asc") {
        "ASC"
    } else {
        "DESC"
    };
    let sql = format!("{SELECT}{where_sql} ORDER BY {sort} {direction}, id DESC");
    Ok(conn
        .prepare(&sql)?
        .query_map(params_from_iter(values), from_row)?
        .collect::<rusqlite::Result<_>>()?)
}
pub fn dashboard(conn: &Connection) -> AppResult<Dashboard> {
    let (total,solved,in_progress,not_started,review)=conn.query_row("SELECT COUNT(*),COALESCE(SUM(CASE WHEN status='solved' THEN 1 ELSE 0 END),0),COALESCE(SUM(CASE WHEN status='in_progress' THEN 1 ELSE 0 END),0),COALESCE(SUM(CASE WHEN status='not_started' THEN 1 ELSE 0 END),0),COALESCE(SUM(CASE WHEN status='review' THEN 1 ELSE 0 END),0) FROM problems", [], |r| Ok((r.get(0)?,r.get(1)?,r.get(2)?,r.get(3)?,r.get(4)?)))?;
    Ok(Dashboard {
        total,
        solved,
        in_progress,
        not_started,
        review,
        recent: recent(conn, 5)?,
        modified: limited(conn, "updated_at DESC", 5)?,
    })
}
pub fn recent(conn: &Connection, limit: i64) -> AppResult<Vec<Problem>> {
    Ok(conn.prepare(&format!("SELECT p.id,p.identifier,p.title,p.description,p.platform,p.url,p.difficulty,p.tags,p.status,p.languages,p.workspace_path,p.notes,p.created_at,p.updated_at,p.last_opened_at FROM recent_problems r INNER JOIN problems p ON p.id=r.problem_id ORDER BY r.opened_at DESC LIMIT ?1"))?.query_map([limit],from_row)?.collect::<rusqlite::Result<_>>()?)
}
pub fn record_open(conn: &Connection, id: i64) -> AppResult<Problem> {
    get(conn, id)?;
    conn.execute(
        "UPDATE problems SET last_opened_at=strftime('%Y-%m-%dT%H:%M:%fZ','now') WHERE id=?1",
        [id],
    )?;
    conn.execute("INSERT INTO recent_problems(problem_id,opened_at) VALUES(?1,strftime('%Y-%m-%dT%H:%M:%fZ','now')) ON CONFLICT(problem_id) DO UPDATE SET opened_at=excluded.opened_at",[id])?;
    get(conn, id)
}
pub fn clear_recent(conn: &Connection) -> AppResult<()> {
    conn.execute("DELETE FROM recent_problems", [])?;
    Ok(())
}
pub fn manifest(problem: &Problem) -> ProblemManifest<'_> {
    ProblemManifest {
        id: Some(problem.id),
        identifier: &problem.identifier,
        title: &problem.title,
        platform: &problem.platform,
        url: &problem.url,
        difficulty: &problem.difficulty,
        tags: &problem.tags,
        status: &problem.status,
        languages: &problem.languages,
        description: &problem.description,
        notes: &problem.notes,
    }
}
fn limited(conn: &Connection, order: &str, limit: i64) -> AppResult<Vec<Problem>> {
    Ok(conn
        .prepare(&format!("{SELECT} ORDER BY {order},id DESC LIMIT ?1"))?
        .query_map([limit], from_row)?
        .collect::<rusqlite::Result<_>>()?)
}
fn available(conn: &Connection, identifier: &str, except: Option<i64>) -> AppResult<()> {
    if identifier.is_empty() {
        return Ok(());
    }
    let existing = conn
        .query_row(
            "SELECT id FROM problems WHERE identifier=?1",
            [identifier],
            |r| r.get(0),
        )
        .optional()?;
    if existing.is_some() && existing != except {
        Err(AppError::Validation(
            "Another problem already uses this identifier".into(),
        ))
    } else {
        Ok(())
    }
}
fn from_row(r: &Row<'_>) -> rusqlite::Result<Problem> {
    let tags: String = r.get(7)?;
    let languages: String = r.get(9)?;
    Ok(Problem {
        id: r.get(0)?,
        identifier: r.get(1)?,
        title: r.get(2)?,
        description: r.get(3)?,
        platform: r.get(4)?,
        url: r.get(5)?,
        difficulty: r.get(6)?,
        tags: decode(&tags, 7)?,
        status: r.get(8)?,
        languages: decode(&languages, 9)?,
        workspace_path: r.get(10)?,
        notes: r.get(11)?,
        created_at: r.get(12)?,
        updated_at: r.get(13)?,
        last_opened_at: r.get(14)?,
    })
}
fn decode(v: &str, column: usize) -> rusqlite::Result<Vec<String>> {
    serde_json::from_str(v).map_err(|e| {
        rusqlite::Error::FromSqlConversionFailure(column, rusqlite::types::Type::Text, Box::new(e))
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::database::schema::initialize_schema;

    fn input(identifier: &str) -> ProblemInput {
        ProblemInput {
            identifier: identifier.into(),
            title: "Two Sum".into(),
            languages: vec!["rust".into()],
            tags: vec!["array".into()],
            ..Default::default()
        }
        .normalized()
        .unwrap()
    }

    #[test]
    fn empty_dashboard_uses_zero_counts() {
        let conn = Connection::open_in_memory().unwrap();
        initialize_schema(&conn).unwrap();
        let dashboard = dashboard(&conn).unwrap();
        assert_eq!(
            (
                dashboard.total,
                dashboard.solved,
                dashboard.in_progress,
                dashboard.not_started,
                dashboard.review
            ),
            (0, 0, 0, 0, 0)
        );
    }

    #[test]
    fn persists_problem_status_and_recent_history() {
        let conn = Connection::open_in_memory().unwrap();
        initialize_schema(&conn).unwrap();
        let problem = create(&conn, input("two-sum"), "/tmp/one").unwrap();
        assert!(create(&conn, input("two-sum"), "/tmp/two").is_err());
        let mut edited = input("two-sum");
        edited.status = "solved".into();
        update(&conn, problem.id, edited).unwrap();
        assert_eq!(dashboard(&conn).unwrap().solved, 1);
        record_open(&conn, problem.id).unwrap();
        assert_eq!(recent(&conn, 10).unwrap()[0].id, problem.id);
        clear_recent(&conn).unwrap();
        assert!(recent(&conn, 10).unwrap().is_empty());
    }
}
