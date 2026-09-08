#[cfg(test)]
mod tests {
    use crate::sql::collect_bind_values;
    use crate::sql::compile_expression;
    use crate::value::BindValue;
    use crate::{Column, Entity, field::Field};

    struct TestUser;
    #[derive(Debug)]
    struct TestUserModel;
    impl Entity for TestUser {
        type Model = TestUserModel;
        const TABLE: &'static str = "users";
        const COLUMNS: &'static [Column] = &[Column::new("id"), Column::new("name")];
    }
    impl TestUser {
        #[allow(non_upper_case_globals)]
        const id: Field<Self, i32> = Field::new("id");
        #[allow(non_upper_case_globals)]
        const name: Field<Self, String> = Field::new("name");
    }
    struct TestPost;
    #[derive(Debug)]
    struct TestPostModel;
    impl Entity for TestPost {
        type Model = TestPostModel;
        const TABLE: &'static str = "posts";
        const COLUMNS: &'static [Column] = &[Column::new("id"), Column::new("title")];
    }
    impl TestPost {
        #[allow(non_upper_case_globals)]
        #[allow(dead_code)]
        const id: Field<Self, i32> = Field::new("id");
        #[allow(non_upper_case_globals)]
        const title: Field<Self, String> = Field::new("title");
    }
    #[test]
    fn user_query_is_generic() {
        let query = TestUser::find().where_(TestUser::name.eq("Fakhir"));
        let sql = query.build_sql();
        assert_eq!(sql, "SELECT id, name FROM users WHERE name = $1");
    }
    #[test]
    fn post_query_is_generic() {
        let query = TestPost::find().where_(TestPost::title.eq("Rust"));
        let sql = query.build_sql();
        assert_eq!(sql, "SELECT id, title FROM posts WHERE title = $1");
    }

    #[test]
    fn integer_field_is_typed() {
        let query = TestUser::find().where_(TestUser::id.eq(10));
        let sql = query.build_sql();
        assert_eq!(sql, "SELECT id, name FROM users WHERE id = $1");
    }

    #[test]
    fn condition_belongs_to_entity() {
        let condition = TestUser::name.eq("Fakhir");
        let query = TestUser::find().where_(condition);
        let sql = query.build_sql();
        assert_eq!(sql, "SELECT id, name FROM users WHERE name = $1");
    }
    #[test]
    fn not_eq_operator_works() {
        let query = TestUser::find().where_(TestUser::id.not_eq(10));
        let sql = query.build_sql();
        assert_eq!(sql, "SELECT id, name FROM users WHERE id <> $1");
    }

    #[test]
    fn gt_operator_works() {
        let query = TestUser::find().where_(TestUser::id.gt(10));
        let sql = query.build_sql();
        assert_eq!(sql, "SELECT id, name FROM users WHERE id > $1");
    }

    #[test]
    fn gte_operator_works() {
        let query = TestUser::find().where_(TestUser::id.gte(10));
        let sql = query.build_sql();
        assert_eq!(sql, "SELECT id, name FROM users WHERE id >= $1");
    }

    #[test]
    fn lt_operator_works() {
        let query = TestUser::find().where_(TestUser::id.lt(10));
        let sql = query.build_sql();
        assert_eq!(sql, "SELECT id, name FROM users WHERE id < $1");
    }

    #[test]
    fn lte_operator_works() {
        let query = TestUser::find().where_(TestUser::id.lte(10));
        let sql = query.build_sql();
        assert_eq!(sql, "SELECT id, name FROM users WHERE id <= $1");
    }
    #[test]
    fn and_condition_works() {
        let condition = TestUser::id.gt(10).and(TestUser::name.eq("Fakhir"));
        let mut next_placeholder = 1;
        let sql = compile_expression(&condition.expression, &mut next_placeholder);
        assert_eq!(sql, "id > $1 AND name = $2");
    }
    #[test]
    fn or_condition_works() {
        let condition = TestUser::id.gt(10).or(TestUser::name.eq("Fakhir"));
        let mut next_placeholder = 1;
        let sql = compile_expression(&condition.expression, &mut next_placeholder);
        assert_eq!(sql, "id > $1 OR name = $2");
    }
    #[test]
    fn order_by_asc_works() {
        let query = TestUser::find().order_by(TestUser::name.asc());
        let sql = query.build_sql();
        assert_eq!(sql, "SELECT id, name FROM users ORDER BY name ASC");
    }

    #[test]
    fn order_by_desc_works() {
        let query = TestUser::find().order_by(TestUser::id.desc());
        let sql = query.build_sql();
        assert_eq!(sql, "SELECT id, name FROM users ORDER BY id DESC");
    }
    #[test]
    fn limit_works() {
        let query = TestUser::find().take(20);
        let sql = query.build_sql();
        assert_eq!(sql, "SELECT id, name FROM users LIMIT 20");
    }

    #[test]
    fn order_by_with_limit_works() {
        let query = TestUser::find().order_by(TestUser::name.asc()).take(20);
        let sql = query.build_sql();
        assert_eq!(sql, "SELECT id, name FROM users ORDER BY name ASC LIMIT 20");
    }

    #[test]
    fn offset_works() {
        let query = TestUser::find().skip(40);
        let sql = query.build_sql();
        assert_eq!(sql, "SELECT id, name FROM users OFFSET 40");
    }

    #[test]
    fn limit_with_offset_works() {
        let query = TestUser::find().take(20).skip(40);
        let sql = query.build_sql();
        assert_eq!(sql, "SELECT id, name FROM users LIMIT 20 OFFSET 40");
    }

    #[test]
    fn order_by_limit_offset_works() {
        let query = TestUser::find()
            .order_by(TestUser::id.asc())
            .take(20)
            .skip(40);

        let sql = query.build_sql();
        assert_eq!(
            sql,
            "SELECT id, name FROM users ORDER BY id ASC LIMIT 20 OFFSET 40"
        );
    }
    #[test]
    fn distinct_works() {
        let query = TestUser::find().distinct();
        let sql = query.build_sql();
        assert_eq!(sql, "SELECT DISTINCT id, name FROM users");
    }

    #[test]
    fn distinct_with_order_by_limit_offset_works() {
        let query = TestUser::find()
            .distinct()
            .order_by(TestUser::name.asc())
            .take(20)
            .skip(40);

        let sql = query.build_sql();

        assert_eq!(
            sql,
            "SELECT DISTINCT id, name FROM users ORDER BY name ASC LIMIT 20 OFFSET 40"
        );
    }

    #[test]
    fn group_by_works() {
        let query = TestUser::find().group_by(TestUser::name.column());
        let sql = query.build_sql();
        assert_eq!(sql, "SELECT id, name FROM users GROUP BY name");
    }
    #[test]
    fn group_by_with_order_by_limit_offset_works() {
        let query = TestUser::find()
            .group_by(TestUser::name.column())
            .order_by(TestUser::name.asc())
            .take(20)
            .skip(40);

        let sql = query.build_sql();

        assert_eq!(
            sql,
            "SELECT id, name FROM users GROUP BY name ORDER BY name ASC LIMIT 20 OFFSET 40"
        );
    }
    #[test]
    fn having_works() {
        let query = TestUser::find()
            .group_by(TestUser::name.column())
            .having(TestUser::id.gt(10));
        let sql = query.build_sql();
        assert_eq!(
            sql,
            "SELECT id, name FROM users GROUP BY name HAVING id > $1"
        );
    }
    #[test]
    fn where_and_having_work_together() {
        let query = TestUser::find()
            .where_(TestUser::id.gt(5))
            .group_by(TestUser::name.column())
            .having(TestUser::id.gt(10));
        let sql = query.build_sql();
        assert_eq!(
            sql,
            "SELECT id, name FROM users WHERE id > $1 GROUP BY name HAVING id > $2"
        );
    }
    #[test]
    fn where_and_having_collect_bind_values_in_order() {
        let query = TestUser::find()
            .where_(TestUser::id.gt(5))
            .group_by(TestUser::name.column())
            .having(TestUser::id.gt(10));
        let mut values = Vec::new();
        if let Some(expression) = &query.statement.where_clause {
            collect_bind_values(expression, &mut values);
        }
        if let Some(expression) = &query.statement.having {
            collect_bind_values(expression, &mut values);
        }
        assert_eq!(values.len(), 2);
        match &values[0] {
            BindValue::I64(value) => assert_eq!(*value, 5),
            _ => panic!("expected first bind value to be i64"),
        }
        match &values[1] {
            BindValue::I64(value) => assert_eq!(*value, 10),
            _ => panic!("expected second bind value to be i64"),
        }
    }
    #[test]
    fn count_select_works() {
        let query = TestUser::find().select(TestUser::id.count());
        let sql = query.build_sql();
        assert_eq!(sql, "SELECT COUNT(id) FROM users");
    }
    #[test]
    fn count_with_group_by_works() {
        let query = TestUser::find()
            .select(TestUser::name.select())
            .select(TestUser::id.count())
            .group_by(TestUser::name.column());

        let sql = query.build_sql();
        assert_eq!(sql, "SELECT name, COUNT(id) FROM users GROUP BY name");
    }
    #[test]
    fn count_with_group_by_and_having_works() {
        let query = TestUser::find()
            .select(TestUser::name.select())
            .select(TestUser::id.count())
            .group_by(TestUser::name.column())
            .having(TestUser::id.count().gt(1));

        let sql = query.build_sql();

        assert_eq!(
            sql,
            "SELECT name, COUNT(id) FROM users GROUP BY name HAVING COUNT(id) > $1"
        );
    }
    #[test]
    fn aggregate_having_collect_bind_value_works() {
        let query = TestUser::find()
            .select(TestUser::name.select())
            .select(TestUser::id.count())
            .group_by(TestUser::name.column())
            .having(TestUser::id.count().gt(1));

        let mut values = Vec::new();

        if let Some(expression) = &query.statement.having {
            collect_bind_values(expression, &mut values);
        }

        assert_eq!(values.len(), 1);

        match &values[0] {
            BindValue::I64(value) => assert_eq!(*value, 1),
            _ => panic!("expected aggregate HAVING bind value to be i64"),
        }
    }
    #[test]
    fn sum_select_works() {
        let query = TestUser::find().select(TestUser::id.sum());
        assert_eq!(query.build_sql(), "SELECT SUM(id) FROM users");
    }
    #[test]
    fn sum_with_where_works() {
        let query = TestUser::find()
            .where_(TestUser::name.eq("Fakhir"))
            .select(TestUser::id.sum());

        assert_eq!(
            query.build_sql(),
            "SELECT SUM(id) FROM users WHERE name = $1"
        );
    }
    #[test]
    fn sum_with_group_by_and_having_works() {
        let query = TestUser::find()
            .select(TestUser::name.select())
            .select(TestUser::id.sum())
            .group_by(TestUser::name.column())
            .having(TestUser::id.sum().gt(2));

        assert_eq!(
            query.build_sql(),
            "SELECT name, SUM(id) FROM users GROUP BY name HAVING SUM(id) > $1"
        );
    }
    #[test]
    fn avg_select_works() {
        let query = TestUser::find().select(TestUser::id.avg());

        assert_eq!(query.build_sql(), "SELECT AVG(id) FROM users");
    }
    #[test]
    fn avg_with_where_works() {
        let query = TestUser::find()
            .where_(TestUser::name.eq("Fakhir"))
            .select(TestUser::id.avg());

        assert_eq!(
            query.build_sql(),
            "SELECT AVG(id) FROM users WHERE name = $1"
        );
    }
    #[test]
    fn avg_with_group_by_works() {
        let query = TestUser::find()
            .select(TestUser::name.select())
            .select(TestUser::id.avg())
            .group_by(TestUser::name.column());

        assert_eq!(
            query.build_sql(),
            "SELECT name, AVG(id) FROM users GROUP BY name"
        );
    }
    #[test]
    fn avg_with_group_by_and_having_works() {
        let query = TestUser::find()
            .select(TestUser::name.select())
            .select(TestUser::id.avg())
            .group_by(TestUser::name.column())
            .having(TestUser::id.avg().gt(2));

        assert_eq!(
            query.build_sql(),
            "SELECT name, AVG(id) FROM users GROUP BY name HAVING AVG(id) > $1"
        );
    }

    #[test]
    fn min_select_works() {
        let query = TestUser::find().select(TestUser::id.min());

        assert_eq!(query.build_sql(), "SELECT MIN(id) FROM users");
    }

    #[test]
    fn max_select_works() {
        let query = TestUser::find().select(TestUser::id.max());

        assert_eq!(query.build_sql(), "SELECT MAX(id) FROM users");
    }
    #[test]
    fn min_with_group_by_works() {
        let query = TestUser::find()
            .select(TestUser::name.select())
            .select(TestUser::id.min())
            .group_by(TestUser::name.column());

        assert_eq!(
            query.build_sql(),
            "SELECT name, MIN(id) FROM users GROUP BY name"
        );
    }

    #[test]
    fn max_with_group_by_works() {
        let query = TestUser::find()
            .select(TestUser::name.select())
            .select(TestUser::id.max())
            .group_by(TestUser::name.column());

        assert_eq!(
            query.build_sql(),
            "SELECT name, MAX(id) FROM users GROUP BY name"
        );
    }

    #[test]
    fn min_with_group_by_and_having_works() {
        let query = TestUser::find()
            .select(TestUser::name.select())
            .select(TestUser::id.min())
            .group_by(TestUser::name.column())
            .having(TestUser::id.min().gt(1));

        assert_eq!(
            query.build_sql(),
            "SELECT name, MIN(id) FROM users GROUP BY name HAVING MIN(id) > $1"
        );
    }

    #[test]
    fn max_with_group_by_and_having_works() {
        let query = TestUser::find()
            .select(TestUser::name.select())
            .select(TestUser::id.max())
            .group_by(TestUser::name.column())
            .having(TestUser::id.max().gt(1));

        assert_eq!(
            query.build_sql(),
            "SELECT name, MAX(id) FROM users GROUP BY name HAVING MAX(id) > $1"
        );
    }
}
