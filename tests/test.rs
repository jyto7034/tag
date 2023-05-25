mod utils;

use tag::{sqlite_wrapper::wrapper::*, utills::object::*};
use utils::util::*;

#[cfg(test)]
mod tests {
    use super::*;

    mod stress {
        use super::*;

        #[allow(unused_must_use)]
        #[ignore]
        #[test]
        fn sql_add_1k_object() {
            let con = Sqlite::new("test.db").unwrap();
            clear_db(&con).unwrap();
            
            add_random_item(&con, 10000).unwrap();
        }
    }

    mod object_test {
        use super::*;

        ///
        /// SQLite 객체 생성 한 다음 태그와 함께 객체를 추가 후
        /// 다시 불러들여와서 추가 한 정보와 동일한지 확인
        ///
        #[test]
        fn sql_write_read_test() {
            let con = Sqlite::new("test.db").unwrap();
            clear_db(&con).unwrap();

            let obj = add_random_item(&con, 4).unwrap();
            let ans = con.read_all_object_with_tags().unwrap();

            assert_eq!(ans[0].path, obj[0].path, "error diff");
            assert_eq!(ans[0].tags, obj[0].tags, "error diff");
        }

        ///
        /// 객체를 단일로 불러옴.
        ///
        #[test]
        fn read_single_object() {
            let con = Sqlite::new("test.db").unwrap();
            clear_db(&con).unwrap();

            let vals = add_random_item(&con, 3).unwrap();

            let ans = con
                .read_single_object(&Tobject {
                    path: vals[0].path.clone(),
                    tags: vec![],
                })
                .unwrap();
            assert_eq!(ans.path, vals[0].path, "error diff");
        }

        ///
        /// 객체를 삭제함
        ///
        #[test]
        fn sql_remove_object() {
            let con = Sqlite::new("test.db").unwrap();
            clear_db(&con).unwrap();

            let obj = add_random_item(&con, 1).unwrap();

            con.remove_object(&obj[0]).unwrap();
        }

        ///
        /// 태그없이 객체를 추가함
        ///
        #[test]
        fn sql_add_object() {
            let con = Sqlite::new("test.db").unwrap();
            clear_db(&con).unwrap();

            con.add_object(&"asd".to_string()[..]).unwrap();
        }

        ///
        /// 객체에 태그를 추가함.
        /// 추가 잘 됐는지 확인해야함.
        ///
        #[test]
        fn sql_search_test() {
            let con = Sqlite::new("test.db").unwrap();
            clear_db(&con).unwrap();

            let objs = vec![
                Tobject::new(
                    "test1".to_string(),
                    ["a", "b", "c"].map(String::from).to_vec(),
                ),
                Tobject::new("test2".to_string(), ["c"].map(String::from).to_vec()),
                Tobject::new("test3".to_string(), ["b", "c"].map(String::from).to_vec()),
            ];
            for i in 0..3 {
                con.add_object_with_tags(&objs[i]).unwrap();
            }

            let ans = con.search(&Tobject::new("st2".to_string(), vec![])).unwrap();
            assert_eq!(ans[0], objs[1], "error diff");
            
            let ans = con.search(&Tobject::new("".to_string(), vec!["b".to_string()])).unwrap();
            assert_eq!(ans[0], objs[0], "error diff");
            assert_eq!(ans[1], objs[2], "error diff");
        }
    }
    mod funcs_test {
        use super::*;
        ///
        /// 설정된 DB 를 생성합니다.
        ///
        #[test]
        fn setup_sqlite_db() {
            Sqlite::new("test.db").unwrap();
        }

        ///
        /// 동일 객체 추가 시 에러를 내는지 확인.
        ///
        #[test]
        #[should_panic = "already exist item"]
        fn sql_already_exist() {
            let con = Sqlite::new("test.db").unwrap();
            clear_db(&con).unwrap();

            let tobj1 = Tobject::new("test1".to_string(), vec![]);
            con.add_object_with_tags(&tobj1).unwrap();
            con.add_object_with_tags(&tobj1).unwrap();
        }
    }
    mod tag_test {
        use super::*;
        ///
        /// 랜덤 객체 3개를 만들어 추가 후, 특정 태그를 DB 에서 제거.
        /// 제거 됐는지 확인절차 만들어야함
        ///
        #[test]
        fn sql_remove_tag_test() {
            let con = Sqlite::new("test.db").unwrap();
            clear_db(&con).unwrap();

            let objs = add_random_item(&con, 3).unwrap();

            let tag = objs[0].tags[0].clone();

            con.remove_tag(&tag[..]).unwrap();
        }

        ///
        /// 태그를 객체로부터 제거함.
        /// 제거 잘 됐는지 확인해야함.
        ///
        #[test]
        fn sql_detach_tag_test() {
            let con = Sqlite::new("test.db").unwrap();
            clear_db(&con).unwrap();

            let objs = add_random_item(&con, 3).unwrap();

            con.detach_tag(&objs[0]).unwrap();
        }

        ///
        /// 객체에 태그를 추가함.
        /// 추가 잘 됐는지 확인해야함.
        ///
        #[test]
        fn sql_attach_tag_test() {
            let con = Sqlite::new("test.db").unwrap();
            clear_db(&con).unwrap();

            let objs = add_random_item(&con, 3).unwrap();

            con.attach_tag(&mut Tobject::new(
                objs[0].path.clone(),
                objs[0].tags.clone(),
            ))
            .unwrap();
        }
    }
}
