

Implementing the ppsat algorithm (https://github.com/PP-FM/ppsat) in garble 



With **tandem** we do

Serverside, do in the /sample directory:

    cargo run --features="bin"


Clientwise, do:


For 4wise clauses

    tandem_http_client tests/ppsat_setup/program.garble4wise.rs --function main --url http://localhost:8000/ --input "( [([true,true,false,false],[false,false,false,false]) , ([false,false,false,false],[true,false,true,false])  , ([false,false,false,false],[false,true,false,true]) , ([false,false,false,false],[false,false,true,true])] , [2usize, 2usize, 2usize, 2usize])" --metadata _


For 6wise clauses:



    tandem_http_client tests/ppsat_setup/program.garble6wise.rs --function main --url http://localhost:8000/ --input "( [ ([false,true,true,false], [false,false,false,false]), ([false,true,false,true],[false,false,false,false]),  ([false,true,false,false],[false,false,false,true]),  ([false,false,false,false], [true,true,true,false]),   ([false,false,true,false],[false,true,false,true]),   ([false,true,true,true],  [true,false,false,false])  ]     , [2usize,2usize,2usize,3usize,3usize,4usize])" --metadata _


---

With **garble** locally we do in the garble repo:

garble run garble_examples/ppsat.garble.rs --function=main "( [ ([true,true,false,false],[false,false,false,false]) ,
           ([false,false,false,false],[true,false,true,false]) , 
           ([false,false,false,false],[false,true,false,true]) ,
           ([false,false,false,false],[false,false,true,true]) ,
           ([false,false,true,false] ,[true,true,false,true])  ,
           ([false,false,false,true],[false,true,true,false])  ]     , [2usize,2usize,2usize,2usize,4usize,3usize])" "( [ ([false,true,true,false], [false,false,false,false]), 
           ([false,true,false,true],[false,false,false,false]), 
           ([false,true,false,false],[false,false,false,true]), 
           ([false,false,false,false], [true,true,true,false]), 
           ([false,false,true,false],[false,true,false,true]), 
           ([false,true,true,true],  [true,false,false,false])  ]     , [2usize,2usize,2usize,3usize,3usize,4usize])"

