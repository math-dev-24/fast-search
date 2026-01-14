pub struct QueryBuilder {
    pub conditions: Vec<String>,
    pub params: Vec<Box<dyn rusqlite::ToSql>>,
    pub cte_conditions: Vec<String>,
    pub cte_params: Vec<Box<dyn rusqlite::ToSql>>,
    pub has_fts: bool,
    pub fts_query: Option<String>,
}

impl QueryBuilder {
    pub fn new() -> Self {
        Self {
            conditions: Vec::new(),
            params: Vec::new(),
            cte_conditions: Vec::new(),
            cte_params: Vec::new(),
            has_fts: false,
            fts_query: None,
        }
    }

    pub fn add_condition(&mut self, condition: String, param: Box<dyn rusqlite::ToSql>) {
        self.conditions.push(condition);
        self.params.push(param);
    }

    pub fn add_simple_condition(&mut self, condition: String) {
        self.conditions.push(condition);
    }

    pub fn add_cte_condition(&mut self, condition: String, params: Vec<Box<dyn rusqlite::ToSql>>) {
        self.cte_conditions.push(condition);
        self.cte_params.extend(params);
    }

    pub fn add_fts_condition(&mut self, query: String) {
        self.has_fts = true;
        self.fts_query = Some(query);
    }

    pub fn build(self, sort_by: &str, sort_order: &str, limit: u32, offset: u32, cursor: Option<i64>) -> (String, Vec<Box<dyn rusqlite::ToSql>>) {
        let mut all_params = Vec::new();

        let cte_prefix = if !self.cte_conditions.is_empty() {
            all_params.extend(self.cte_params);
            format!("WITH roots(root) AS (VALUES {}) ",
                    self.cte_conditions.join(", "))
        } else {
            String::new()
        };

        let mut where_clause = if self.conditions.is_empty() {
            "1=1".to_string()
        } else {
            self.conditions.join(" AND ")
        };

        // 2. FTS query parameter (for MATCH clause in WHERE)
        // Must come after CTE params but before cursor and regular WHERE params
        if let Some(fts_query) = self.fts_query {
            all_params.push(Box::new(fts_query) as Box<dyn rusqlite::ToSql>);
        }

        // 3. Cursor parameter (for files.id > ? in WHERE clause)
        // Must come after FTS query but before regular WHERE params
        if let Some(cursor_id) = cursor {
            let cursor_condition = if where_clause == "1=1" {
                format!("files.id > ?")
            } else {
                format!("{} AND files.id > ?", where_clause)
            };
            where_clause = cursor_condition;
            all_params.push(Box::new(cursor_id) as Box<dyn rusqlite::ToSql>);
        }

        // 4. Regular WHERE condition parameters (for other WHERE conditions)
        all_params.extend(self.params);

        // Utiliser cursor-based pagination si cursor est fourni, sinon fallback sur OFFSET
        let pagination = if cursor.is_some() {
            format!("LIMIT {}", limit)
        } else {
            format!("LIMIT {} OFFSET {}", limit, offset)
        };

        let sql = if self.has_fts {
            format!(
                "{}SELECT files.* FROM files \
                 JOIN fts_content ON files.id = fts_content.file_id \
                 WHERE fts_content.content MATCH ? AND {} \
                 ORDER BY bm25(fts_content) ASC, files.{} {} {}",
                cte_prefix, where_clause, sort_by, sort_order, pagination
            )
        } else {
            format!(
                "{}SELECT * FROM files \
                 WHERE {} \
                 ORDER BY {} {} {}",
                cte_prefix, where_clause, sort_by, sort_order, pagination
            )
        };

        (sql, all_params)
    }
}