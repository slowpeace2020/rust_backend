import Debug "mo:base/Debug";
import Principal "mo:base/Principal";

actor class Avatar (url: Text, owner: Principal, content: [Nat8]) = this {
  
  private let imgUrl = url;
  private var principalId = owner;
  private let imageBytes = content;

  public query func getImgUrl() : async Text{
    return imgUrl;
  };

  public query func getOwner() : async Principal {
    return principalId;
  };

  public query func getImageBytes() : async [Nat8] {
    return imageBytes;
  };

  public query func getCanisterId() : async Principal {
    return Principal.fromActor(this);
  };

};