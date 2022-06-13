import Principal "mo:base/Principal";
import HashMap "mo:base/HashMap";
import Debug "mo:base/Debug";
import Iter "mo:base/Iter";
import List "mo:base/List";

import UserProfile "canister:user_profile";


actor Token {

  let owner : Principal = Principal.fromText("4p27i-ym6ti-34gws-3evnd-k6hdp-5iv6a-g7hzw-nc33q-lpyqi-elhkz-tae");
  let totalSupply : Nat = 1000000000;
  let symbol : Text = "CHUSHAN";

  private type TransferRecord = {
        itemOwnerA: Principal;
        itemOwnerB: Text;
        amount: Nat;
  };


  private stable var balanceEntries : [(Principal, Nat)] = [];
  private var balances = HashMap.HashMap<Principal, Nat>(1, Principal.equal, Principal.hash);
  private  var mapOfTransferOwners = HashMap.HashMap<Principal, List.List<TransferRecord>>(1, Principal.equal, Principal.hash);

  if (balances.size() < 1) {
    balances.put(owner, totalSupply);
  };
    
  public query func balanceOf(who: Principal) : async Nat {

    let balance : Nat = switch (balances.get(who)) {
      case null 0;
      case (?result) result;
    };

    return balance;
  };

  public query func getSymbol() : async Text {
    return symbol;
  };

  public shared(msg) func payOut() : async Text {
    Debug.print(debug_show(msg.caller));
    if (balances.get(msg.caller) == null) {
      let amount = 10000;
      let result = await transfer(msg.caller, amount);
      return result;
    } else {
      return "Already Claimed"
    }
  };

  public shared(msg) func transfer(to: Principal, amount: Nat) : async Text {
    let fromBalance = await balanceOf(msg.caller);
    Debug.print("fromBalance: ");
    Debug.print(debug_show(msg.caller));
    if (fromBalance > amount) {
      let newFromBalance : Nat = fromBalance - amount;
      balances.put(msg.caller, newFromBalance);

      let toBalance = await balanceOf(to);
      let newToBalance = toBalance + amount;
      balances.put(to, newToBalance);

      return "Success";
    } else {
      return "Insufficient Funds"
    }
  };


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

  //todo account withdraw
  public shared(msg) withDraw(owner:Text) : async Text{
   let userInfo = UserProfile.getProfileByName(owner);
   return "success";
  }

  system func preupgrade() {
    balanceEntries := Iter.toArray(balances.entries());
  };

  system func postupgrade() {
    balances := HashMap.fromIter<Principal, Nat>(balanceEntries.vals(), 1, Principal.equal, Principal.hash);
    if (balances.size() < 1) {
      balances.put(owner, totalSupply);
    };
  };

};


