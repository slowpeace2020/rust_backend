```
  public shared(msg) func redEnvelopeTransfer(ownerA: Principal,ownerB: Text, amount: Nat) : async Text {
     //if (amount%2!=0){
      //    amount = amount - 1;
     //}

     //if (amount<1){
      //return "amount less than 2, couldn't transfer it".
     //}

     let saveToPublicAccount : Text = await transfer(owner,amount);

     if(saveToPublicAccount=="Success"){
        let newtransferRecord : TransferRecord = {
                 itemOwnerA = ownerA;
                 itemOwnerB = ownerB;
                 amount = amount;
           };


          var ownedRecords : List.List<TransferRecord> = switch (mapOfTransferOwners.get(owner)) {
                 case null List.nil<TransferRecord>();
                 case (?result) result;
           };

          ownedRecords := List.push(newtransferRecord, ownedRecords);
          mapOfTransferOwners.put(ownerA, ownedRecords);
          return saveToPublicAccount;
     }else{
          return saveToPublicAccount;
     }

  };

```

增加字段，旧数据结构对不上
 table: 
 type table0 = 
    record { 23_515 : nat64; 3_331_805_203 : table1 }\n       
    type table1 = vec table2\n       
    type table2 = 
    record {\n         23_515 : nat64;\n         
    458_051_046 : bool;\n         
    1_291_439_277 : text;\n         
    2_049_725_626 : text;\n         
    2_781_795_542 : nat64;\n         
    3_611_914_462 : text;\n       
    }\n       
    wire_type: table0, 
    expect_type: record {\n         id : nat64;\n         
    posts : vec record {\n           id : nat64;\n           
    is_invited : bool;\n           \"text\" : text;\n           
    user_other_name : text;\n           
    user_self_id : text;\n           
    timestamp : nat64;\n           
    user_other_id : text;\n         
    };\n       
    }\n    
    1: table0 is not a subtype of 
    record {\n         id : nat64;\n         
    posts : vec record {\n           
    id : nat64;\n           
    is_invited : bool;\n           
    \"text\" : text;\n           
    user_other_name : text;\n           
    user_self_id : text;\n           
    timestamp : nat64;\n           
    user_other_id : text;\n         
    };\n       }\n    
    2: Record field posts: 
    table1 is not a subtype of vec 
    record {\n         
    id : nat64;\n         
    is_invited : bool;\n         
    \"text\" : text;\n         
    user_other_name : text;\n         
    user_self_id : text;\n         
    timestamp : nat64;\n         
    user_other_id : text;\n       
    }\n    
    3: Record field 
    user_other_name: text is only in the expected type 
    and is not of opt or reserved type)"', 
    src/post_service/src/actor.rs:79:69"
