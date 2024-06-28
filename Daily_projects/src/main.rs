use csv;

fn csv_reader(path:&str)->bool{
    let csv_res=csv::Reader::from_path(path);
    match csv_res{
        Ok(mut reader)=>{
             for rec in reader.records(){
                match rec {
                    Ok(res)=>{
                        println!("{:?}",res);
                    },
                    Err(e)=>{ 
                        eprintln!("{}",e);
                        return false;

                    }
                    
                }
               
             }
             return  true;
        },
        Err(e) =>{
            eprintln!("{}",e);
            return false;
        }
    }
}

fn main(){
    let path:&str="./test1.csv";
    let read_status=csv_reader(path);
    if read_status{
    println!("CSV file read successfully");
    }else{
        eprintln!("Failed to read CSV file!!Try again");
    }

}