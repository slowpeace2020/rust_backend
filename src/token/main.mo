import Principal "mo:base/Principal";
import HashMap "mo:base/HashMap";
import Text "mo:base/Text";
import Debug "mo:base/Debug";
import Iter "mo:base/Iter";
import List "mo:base/List";

import UserProfile "canister:user_profile";


actor Token {

  let owner : Principal = Principal.fromText("4p27i-ym6ti-34gws-3evnd-k6hdp-5iv6a-g7hzw-nc33q-lpyqi-elhkz-tae");
  let totalSupply : Nat = 1000000000;
  let symbol : Text = "CHUSHAN";


  private stable var balanceEntries : [(Principal, Nat)] = [];
  private var balances = HashMap.HashMap<Principal, Nat>(1, Principal.equal, Principal.hash);

  private var withdrawBalances = HashMap.HashMap<Principal, Nat>(1, Principal.equal, Principal.hash);
  private var nameBalances = HashMap.HashMap<Text, Nat>(10, Text.equal, Text.hash);

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

  //for red Envelope Transfer
  public shared(msg) func redTransfer(from: Principal,to: Principal, amount: Nat) : async Text {
      Debug.print("fromBalance: ");
      Debug.print(debug_show(from));
       Debug.print("msg.caller: ");
       Debug.print(debug_show(msg.caller));
       //if(from == msg.caller){
        let fromBalance = await balanceOf(from);
        if (fromBalance > amount) {
          let newFromBalance : Nat = fromBalance - amount;
          balances.put(from, newFromBalance);

          let toBalance = await balanceOf(to);
          let newToBalance = toBalance + amount;
          balances.put(to, newToBalance);

          return "Success";
        } else {
          return "Insufficient Funds"
        }
       //}else{
        //return "identity does not match"
       //}
     };


  public shared(msg) func redEnvelopeTransfer(ownerA: Principal,ownerB: Text, amount: Nat) : async Text {
     Debug.print("who is sending redEnvelopeTransfer: ");
     Debug.print(debug_show(msg.caller));
     let saveToPublicAccount : Text = await redTransfer(msg.caller,owner,amount);

     if(saveToPublicAccount=="Success"){
         var item : Nat = switch (withdrawBalances.get(ownerA)) {
             case(null) 0;
             case (?result) result;
           };
          item +=amount/2;
          withdrawBalances.put(ownerA,item);

         var reminderB : Nat = switch (nameBalances.get(ownerB)) {
             case(null) 0;
             case (?result) result;
           };
           reminderB +=amount/2;
           nameBalances.put(ownerB,reminderB);
           return saveToPublicAccount;
     }else{
          return saveToPublicAccount;
     }

  };

    //todo account withdraw
    public shared(msg) func withDraw(owner:Text, amount: Nat) : async Text {
       //var userInfo : UserProfile.Profile = switch (UserProfile.getProfileByName(owner)){
       //   case null return "the user does not exist";
       //   case (?result) result
       // };
       var userInfo : ?Principal = await UserProfile.getPrincipalByName(owner);
       //Debug.print("withDraw userInfo : ");
       //Debug.print(debug_show(userInfo));
       //Debug.print(userInfo);
       return "success";
    };

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


