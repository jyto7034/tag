use crate::enums::defs;
use crate::utills::funcs::*;
use crate::utills::object::Tobject;
use sqlite::State;

#[allow(dead_code)]
pub struct Sqlite {
    object: sqlite::Connection,
}

type Resultx<T> = Result<T, sqlite::Error>;
mod funcs {
    use super::*;
    impl Sqlite {
        pub fn test(){
            println!("asd");
        }

        ///
        /// db 의 경로를 인자로 받은 뒤 해당 db 의 객체를 반환합니다.
        ///
        pub fn new<T: AsRef<std::path::Path>>(path: T) -> Resultx<Sqlite> {
            let obj = sqlite::open(path)?;
            let query = "CREATE TABLE data (path TEXT, tags TEXT);";
            match obj.execute(query) {
                Ok(_) => (),
                Err(ref error) if error.code.expect("te") == defs::TABLE_ALREADY_EXISTS => {
                    println!()
                }
                Err(error) => {
                    panic!("Unknown : {}", error);
                }
            }
            Ok(Sqlite { object: obj })
        }

        ///
        /// 쿼리를 실행합니다.
        ///
        pub fn execute<T: AsRef<str>>(&self, query: T) -> Resultx<()> {
            self.object.execute(query)?;
            Ok(())
        }

        pub fn get_obejct(&self) -> Resultx<&sqlite::Connection> {
            Ok(&self.object)
        }
    }
}

mod objects {
    use super::*;
    impl Sqlite {
        ///
        /// 객체-태그 쌍을 추가합니다.
        /// 이미 존재한다면 실패합니다.
        ///
        pub fn add_object_with_tags(&self, obj: &Tobject) -> Resultx<()> {
            let path = &obj.path;
            let tags = &obj.tags.join(",");

            let query = format!("INSERT INTO data VALUES ('{}', '{}')", path, tags);

            let res = self.read_single_object(obj)?;
            if res.path.eq(path) {
                return Err(sqlite::Error {
                    code: Some(99),
                    message: Some("already exist item".to_string()),
                });
            }
            self.execute(query)?;
            Ok(())
        }

        ///
        /// 객체를 DB 로부터 삭제합니다.
        ///
        pub fn remove_object(&self, tobj: &Tobject) -> Resultx<()> {
            let query = format!("DELETE from data where path= '{}'", tobj.path);
            self.object.execute(query)?;
            Ok(())
        }

        ///
        /// 객체를 DB 에 추가합니다
        ///
        pub fn add_object(&self, path: &str) -> Resultx<()> {
            let query = format!("INSERT INTO data VALUES ('{}', '')", path);
            self.object.execute(query)?;
            Ok(())
        }

        ///
        /// 모든 객체-태그 정보를 불러옵니다
        ///
        pub fn read_all_object_with_tags(&self) -> Resultx<Vec<Tobject>> {
            let query = "SELECT * FROM data";
            let mut state = self.object.prepare(query)?;
            let mut obj = vec![];
            while let Ok(State::Row) = state.next() {
                obj.push(Tobject {
                    path: String::from(state.read::<String, _>("path")?),
                    tags: str_to_vec(state.read::<String, _>("tags")?),
                });
            }
            Ok(obj)
        }

        ///
        /// 단일 객체-태그 정보를 불러옵니다
        ///
        pub fn read_single_object(&self, tobj: &Tobject) -> Resultx<Tobject> {
            let (_type, data) = if tobj.path.len() != 0 {
                ("path", tobj.path.to_string())
            } else {
                ("tags", tobj.tags.join(","))
            };
            let query = format!("SELECT * from data where {} = '{}'", _type, data);
            let mut state = self.object.prepare(query)?;
            let mut obj = Tobject::new(String::new(), vec![]);

            if let Ok(State::Row) = state.next() {
                obj.path = state.read::<String, _>("path")?;
                obj.tags = str_to_vec(state.read::<String, _>("tags")?);
            }

            Ok(obj)
        }

        ///
        /// 단일 객체-태그 정보를 불러옵니다
        ///
        pub fn search(&self, tobj: &Tobject) -> Resultx<Vec<Tobject>> {
            let mut query = "SELECT * FROM data WHERE tags LIKE '%%'".to_string();
            if tobj.path.len() != 0 {
                query = format!("SELECT * from data where path LIKE '%{}%'", tobj.path);
            } else {
                for item in &tobj.tags {
                    query = format!("{} and tags LIKE '%{}%'", query, item);
                }
            }

            let mut state = self.object.prepare(query)?;
            let mut ans = vec![];

            while let Ok(State::Row) = state.next() {
                let path = state.read::<String, _>("path")?;
                let tags = str_to_vec(state.read::<String, _>("tags")?);
                ans.push(Tobject::new(path, tags));
            }

            Ok(ans)
        }
    }
}

mod tags {
    use super::*;
    impl Sqlite {
        ///
        /// 인자로 들어온 태그를 DB 에서 삭제합니다.
        ///
        pub fn remove_tag(&self, tag_name: &str) -> Resultx<()> {
            let query = format!("update data set tags = replace(tags, '{}', '')", tag_name);
            self.object.execute(query)?;
            Ok(())
        }

        ///
        /// 객체로부터 tag를 삭제합니다.
        ///
        pub fn detach_tag(&self, obj: &Tobject) -> Resultx<()> {
            let query = format!(
                "update data set tags = replace(tags, '{}', '') where path = '{}'",
                obj.tags[0], obj.path
            );
            self.object.execute(query)?;
            Ok(())
        }

        ///
        /// 객체에 tag를 추가합니다.
        ///
        pub fn attach_tag(&self, tobj: &mut Tobject) -> Resultx<()> {
            // 인자로 넘어온 tobj 의 path 를 참조하여 객체 하나를 obj 에 저장합니다.
            let mut obj = self.read_single_object(&Tobject::new(tobj.path.clone(), vec![]))?;

            // 불러온 obj 의 tag 정보를 tobj 로 옮깁니다.
            tobj.tags.append(&mut obj.tags);

            // format! 매크로를 통해 새롭게 업데이트된 태그 정보로 db 를 업데이트 해줍니다.
            let query = format!(
                "update data set tags = '{}' where path = '{}'",
                tobj.tags.join(","),
                tobj.path
            );

            self.object.execute(query)?;
            Ok(())
        }
    }
}

/*

    몬헌 월드
    아웃라스트



    public:
    리틀나이트메어
    It takes two
*/