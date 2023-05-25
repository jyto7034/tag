use tag::{sqlite_wrapper::wrapper::*, utills::funcs::*, utills::object::*};

type Resultx<T> = Result<T, sqlite::Error>;

pub fn add_random_item(con: &Sqlite, cnt: usize) -> Resultx<Vec<Tobject>> {
    let mut v = vec![];

    for _i in 0..cnt {
        let val = Tobject {
            path: String::from(gen_str(4)),
            tags: vec![gen_str(3), gen_str(3), gen_str(3), gen_str(3)],
        };
        con.add_object_with_tags(&val)?;
        v.push(Tobject::clone(&val));
    }
    Ok(v)
}

pub fn clear_db(con: &Sqlite) -> Resultx<()> {
    con.execute("delete from data")?;
    Ok(())
}
