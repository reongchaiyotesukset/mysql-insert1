use std::rc::Rc;

use mysql::*;
use mysql::prelude::*;


#[derive(Debug, PartialEq, Eq)]
struct example {
    id: i32,
    data: Option<String>,
}
fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {



         
            let url = "mysql://root:12345678@localhost:3306/testdb";
            //let url = "mysql://root:Root45Ddf23#%^%&@784.168.1.43:3306/testdb";
			let mut pool = Pool::new(url)?;
			let mut conn = pool.get_conn()?;
             
       
             
             let insertdata = vec![
				example { id: 1, data: Some("c#".into()) },
			];

		


			//test
			   conn.exec_batch(
			"INSERT INTO example (id,  data)
            VALUES (:id, :data)",
            insertdata.iter().map(|p| params! {
            "id" => p.id,
            "data" => &p.data,
				})
			)?;
			
			//println!("{:?}",stack[1].data);
		    //println!("{:?}",data);
      
      
;
       
  
       Ok(())
    
}
 
