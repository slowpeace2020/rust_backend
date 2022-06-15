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