var searchIndex = JSON.parse('{\
"buzz":{"doc":"A rust web framework that avoids dependancies wherever …","t":[2,0,0,2,13,13,13,3,4,13,13,13,13,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,12,12,12,12,12,12,3,3,24,3,3,8,11,11,11,11,11,11,11,11,23,11,11,11,11,11,11,11,11,11,23,11,12,11,11,11,11,11,11,11,11,23,23,23,23,11,10,11,11,14,11,11,11,11,11,11,11,11,11,11,11,11,11],"n":["codegen","json","prelude","types","Array","Bool","Fraction","Json","JsonValue","Null","Number","Object","String","borrow","borrow","borrow_mut","borrow_mut","clone","clone_into","deref","eq","fmt","from","from","from_body","get","into","into","parse","to_owned","to_string","try_from","try_from","try_into","try_into","type_id","type_id","0","0","0","0","0","0","Buzz","BuzzContext","Deserialize","Inject","InjectMut","Respond","borrow","borrow","borrow","borrow","borrow_mut","borrow_mut","borrow_mut","borrow_mut","delete","deref","deref","deref_mut","dispatch","from","from","from","from","get","get","get_mut","headers","into","into","into","into","middleware","new","new","new","options","patch","post","put","register","respond","router","routes","routes","run_server","try_from","try_from","try_from","try_from","try_into","try_into","try_into","try_into","type_id","type_id","type_id","type_id"],"q":["buzz","","","","buzz::json","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","buzz::json::JsonValue","","","","","","buzz::prelude","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","",""],"d":["","","","","","","","","","","","","","","","","","","","","","","Returns the argument unchanged.","Returns the argument unchanged.","","","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","","","","","","","","","","","","","","","","","Holds metadata about the incoming <code>HttpRequest</code> that’s …","","Wrapper that indicates a shared reference should be …","Wrapper that indicates an exclusive reference should be …","","","","","","","","","","","","","","","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","","","","","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","","","","","","","","","","","","","","","","","","","","","","","","","",""],"i":[0,0,0,0,1,1,1,0,0,1,1,1,1,2,1,2,1,1,1,2,1,1,2,1,2,2,2,1,1,1,1,2,1,2,1,2,1,21,22,23,24,25,26,0,0,0,0,0,0,16,27,14,15,16,27,14,15,0,14,15,15,16,16,27,14,15,14,0,15,27,16,27,14,15,16,16,14,15,0,0,0,0,16,28,16,16,0,16,16,27,14,15,16,27,14,15,16,27,14,15],"f":[0,0,0,0,0,0,0,0,0,0,0,0,0,[[]],[[]],[[]],[[]],[1,1],[[]],[2],[[1,1],3],[[1,4],5],[[]],[[]],[6,[[10,[[2,[[7,[1]]]],[9,[8]]]]]],[2],[[]],[[]],[6,[[10,[1,11]]]],[[]],[1,12],[[],10],[[],10],[[],10],[[],10],[[],13],[[],13],0,0,0,0,0,0,0,0,0,0,0,0,[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],0,[14],[15],[15],[[16,17],18],[[]],[[]],[[]],[[]],[14],0,[15],0,[[]],[[]],[[]],[[]],[[16,19],16],[6,16],[[],14],[[],15],0,0,0,0,[16,16],[[],18],[16,16],[[16,20],16],0,[16],[[],10],[[],10],[[],10],[[],10],[[],10],[[],10],[[],10],[[],10],[[],13],[[],13],[[],13],[[],13]],"p":[[4,"JsonValue"],[3,"Json"],[15,"bool"],[3,"Formatter"],[6,"Result"],[15,"str"],[8,"Deserialize"],[8,"Error"],[3,"Box"],[4,"Result"],[4,"JsonParseError"],[3,"String"],[3,"TypeId"],[3,"Inject"],[3,"InjectMut"],[3,"Buzz"],[3,"HttpRequest"],[3,"HttpResponse"],[6,"Middleware"],[3,"Vec"],[13,"Number"],[13,"Fraction"],[13,"Bool"],[13,"String"],[13,"Array"],[13,"Object"],[3,"BuzzContext"],[8,"Respond"]]},\
"buzz_codegen":{"doc":"","t":[24,23,23,23,23,23,23,14],"n":["Deserialize","delete","get","options","patch","post","put","routes"],"q":["buzz_codegen","","","","","","",""],"d":["","","","","","","",""],"i":[0,0,0,0,0,0,0,0],"f":[0,0,0,0,0,0,0,0],"p":[]},\
"buzz_types":{"doc":"This crate contains common types that are used by multiple …","t":[13,3,13,13,13,6,3,4,3,3,4,13,3,3,13,6,13,13,13,13,13,13,13,3,3,13,4,13,11,12,12,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,12,11,11,11,11,11,11,11,11,12,12,11,11,11,0,11,11,11,0,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,12,12,12,12,11,11,11,11,11,11,11,11,11,11,11,12,12,12,11,11,11,11,12,12,12,12,11,12,11,11,11,11,11,11,0,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,12,12,12,3,3,11,11,11,11,11,11,12,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,13,13,4,4,13,13,13,13,13,13,4,13,4,13,13,13,13,13,13,13,13,4,13,13,13,13,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,8,8,8,10,10,10],"n":["BadRequest","BuzzContext","Const","Delete","Get","Handler","Headers","HttpMethod","HttpRequest","HttpResponse","HttpStatusCode","ImATeapot","Inject","InjectMut","InternalServerError","Middleware","NoContent","NotFound","Ok","Options","Patch","Post","Put","Route","RouteMetadata","SegNone","SegmentType","Variable","body","body","body","borrow","borrow","borrow","borrow","borrow","borrow","borrow","borrow","borrow","borrow","borrow","borrow_mut","borrow_mut","borrow_mut","borrow_mut","borrow_mut","borrow_mut","borrow_mut","borrow_mut","borrow_mut","borrow_mut","borrow_mut","children","clone","clone","clone","clone","clone_into","clone_into","clone_into","clone_into","content_length","content_type","deref","deref","deref_mut","dev","eq","eq","eq","errors","fmt","fmt","fmt","fmt","fmt","fmt","fmt","from","from","from","from","from","from","from","from","from","from","from","from_iter","from_str","from_vec","get","get","get_mut","handler","headers","headers","headers","into","into","into","into","into","into","into","into","into","into","into","method","method","method","new","new","new","new","path","remaining","route","segment","status","status_code","to_owned","to_owned","to_owned","to_owned","to_string","to_string","traits","try_from","try_from","try_from","try_from","try_from","try_from","try_from","try_from","try_from","try_from","try_from","try_into","try_into","try_into","try_into","try_into","try_into","try_into","try_into","try_into","try_into","try_into","type_id","type_id","type_id","type_id","type_id","type_id","type_id","type_id","type_id","type_id","type_id","version","0","0","DependancyInjection","Parser","borrow","borrow","borrow_mut","borrow_mut","consume","consume_while","data","fmt","from","from","get","into","into","new","new","offset","peek","register","remaining","rewind","subbytes","subbytes_to_offset","substr","substr_to_offset","take","take_if","take_n","try_from","try_from","try_into","try_into","type_id","type_id","BadRequest","BodyParseError","BuzzError","DeserializationError","DuplicateDecimals","EndOfInputWhile","ExpectedColon","ExpectedComma","FractionalParseError","Header","HttpParseError","InvalidObjectKey","JsonParseError","LockAcquirePoisoned","Method","MismatchedTypes","MissingLeadingSlash","MissingNewlineAfterHeaders","MissingValues","NumberParseError","Path","RouteParseError","UnexpectedToken","UseOfUnregesteredInject","VersionParse","VersionText","borrow","borrow","borrow","borrow","borrow","borrow_mut","borrow_mut","borrow_mut","borrow_mut","borrow_mut","fmt","fmt","fmt","fmt","fmt","fmt","fmt","fmt","fmt","fmt","from","from","from","from","from","from","from","into","into","into","into","into","provide","provide","provide","provide","provide","source","source","source","to_string","to_string","to_string","to_string","to_string","try_from","try_from","try_from","try_from","try_from","try_into","try_into","try_into","try_into","try_into","type_id","type_id","type_id","type_id","type_id","0","0","0","0","1","0","0","1","1","0","0","0","0","0","0","0","0","0","0","Deserialize","FromBody","Respond","deserialize","from_body","respond"],"q":["buzz_types","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","buzz_types::SegmentType","","buzz_types::dev","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","buzz_types::errors","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","buzz_types::errors::BuzzError","","","","","buzz_types::errors::DeserializationError","","","","buzz_types::errors::HttpParseError","","","","","buzz_types::errors::JsonParseError","","","","","buzz_types::traits","","","","",""],"d":["","Holds metadata about the incoming <code>HttpRequest</code> that’s …","A static segment of a URL that will only match it’s …","","","A wrapper around a user defined route handler.","","Represents an HTTP request method.","Representation of an HTTP request that’s been parsed.","Holds the data that will be formatted back into an HTTP …","Represents the status code of an HTTP response.","","Wrapper that indicates a shared reference should be …","Wrapper that indicates an exclusive reference should be …","","A function that can modify a request before it’s handled …","","","","","","","","Represents a tree of url routes.","Contains preparsed route segments and the HTTP method that …","A placeholder for empty or terminal segments","Represents the type of a url segment.","A dynamic segment of the URL that can be passed in as a …","","","","","","","","","","","","","","","","","","","","","","","","","","Recursively contains routes that share this segment as a …","","","","","","","","","","","","","","Contains things are used internally by Buzz across crates …","","","","Contains error types","","","","","","","","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","","","","","","","The function that should be called when this route is hit.","","","","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","","","If the route is valid to be hit (see <code>Route::handler</code> for …","","","","","","","","Denotes the type of this portion of the url.","","","","","","","","","Contains traits","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","Returns the argument unchanged.","Returns the argument unchanged.","","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","Returns the argument unchanged.","Returns the argument unchanged.","","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","",""],"i":[4,0,5,3,3,0,0,0,0,0,0,4,0,0,4,0,4,4,4,3,3,3,3,0,0,5,0,5,1,12,1,34,35,7,8,3,12,1,4,5,6,13,34,35,7,8,3,12,1,4,5,6,13,6,3,4,5,6,3,4,5,6,13,13,7,8,8,0,3,4,5,0,3,12,1,4,5,6,13,34,35,7,8,3,12,1,4,5,6,13,13,3,6,7,13,8,6,35,12,1,34,35,7,8,3,12,1,4,5,6,13,34,12,6,7,8,1,6,12,13,34,6,1,1,3,4,5,6,3,4,0,34,35,7,8,3,12,1,4,5,6,13,34,35,7,8,3,12,1,4,5,6,13,34,35,7,8,3,12,1,4,5,6,13,12,36,37,0,0,22,20,22,20,20,20,20,20,22,20,22,22,20,22,20,20,20,22,20,20,20,20,20,20,20,20,20,22,20,22,20,22,20,27,27,0,0,28,28,28,28,28,25,0,28,0,27,25,29,26,25,29,28,25,0,28,27,25,25,25,26,27,28,29,25,26,27,28,29,25,25,26,26,27,27,28,28,29,29,25,25,26,27,27,28,29,25,26,27,28,29,25,26,27,28,29,25,27,28,25,26,27,28,29,25,26,27,28,29,25,26,27,28,29,25,26,27,28,29,38,39,40,41,41,42,43,42,43,44,45,46,47,48,49,50,51,52,53,0,0,0,54,55,56],"f":[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,[[1,2],1],0,0,[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],0,[3,3],[4,4],[5,5],[6,6],[[]],[[]],[[]],[[]],0,0,[7],[8],[8],0,[[3,3],9],[[4,4],9],[[5,5],9],0,[[3,10],11],[[12,10],11],[[1,10],11],[[4,10],11],[[5,10],11],[[6,10],11],[[13,10],11],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[14,13],[15,[[16,[3]]]],[[3,17],6],[7],[[13,15],[[18,[15]]]],[8],0,0,0,0,[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],0,0,0,[[],7],[[],8],[4,1],[[],6],0,0,0,0,[[1,4],1],0,[[]],[[]],[[]],[[]],[3,2],[4,2],0,[[],16],[[],16],[[],16],[[],16],[[],16],[[],16],[[],16],[[],16],[[],16],[[],16],[[],16],[[],16],[[],16],[[],16],[[],16],[[],16],[[],16],[[],16],[[],16],[[],16],[[],16],[[],16],[[],19],[[],19],[[],19],[[],19],[[],19],[[],19],[[],19],[[],19],[[],19],[[],19],[[],19],0,0,0,0,0,[[]],[[]],[[]],[[]],[[20,21]],[20],0,[[20,10],11],[[]],[[]],[22,[[18,[23]]]],[[]],[[]],[[],22],[[],20],[20,21],[20,[[18,[24]]]],[22],[20,21],[[20,21]],[[20,21,21]],[[20,21]],[[20,21,21],15],[[20,21],15],[20,[[18,[24]]]],[20,[[18,[24]]]],[[20,21],18],[[],16],[[],16],[[],16],[[],16],[[],19],[[],19],0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[25,10],11],[[25,10],11],[[26,10],11],[[26,10],11],[[27,10],11],[[27,10],11],[[28,10],11],[[28,10],11],[[29,10],11],[[29,10],11],[30,25],[[]],[[]],[[[32,[31]]],27],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[33],[33],[33],[33],[33],[25,[[18,[31]]]],[27,[[18,[31]]]],[28,[[18,[31]]]],[[],2],[[],2],[[],2],[[],2],[[],2],[[],16],[[],16],[[],16],[[],16],[[],16],[[],16],[[],16],[[],16],[[],16],[[],16],[[],19],[[],19],[[],19],[[],19],[[],19],0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,[[],[[16,[29]]]],[15,[[16,[[32,[31]]]]]],[[],1]],"p":[[3,"HttpResponse"],[3,"String"],[4,"HttpMethod"],[4,"HttpStatusCode"],[4,"SegmentType"],[3,"Route"],[3,"Inject"],[3,"InjectMut"],[15,"bool"],[3,"Formatter"],[6,"Result"],[3,"HttpRequest"],[3,"Headers"],[3,"Vec"],[15,"str"],[4,"Result"],[6,"Handler"],[4,"Option"],[3,"TypeId"],[3,"Parser"],[15,"usize"],[3,"DependancyInjection"],[3,"RwLock"],[15,"u8"],[4,"HttpParseError"],[4,"RouteParseError"],[4,"BuzzError"],[4,"JsonParseError"],[4,"DeserializationError"],[3,"ParseFloatError"],[8,"Error"],[3,"Box"],[3,"Demand"],[3,"RouteMetadata"],[3,"BuzzContext"],[13,"Const"],[13,"Variable"],[13,"UseOfUnregesteredInject"],[13,"BodyParseError"],[13,"BadRequest"],[13,"LockAcquirePoisoned"],[13,"MismatchedTypes"],[13,"MissingValues"],[13,"Method"],[13,"Path"],[13,"VersionText"],[13,"VersionParse"],[13,"Header"],[13,"UnexpectedToken"],[13,"EndOfInputWhile"],[13,"InvalidObjectKey"],[13,"NumberParseError"],[13,"FractionalParseError"],[8,"Deserialize"],[8,"FromBody"],[8,"Respond"]]}\
}');
if (typeof window !== 'undefined' && window.initSearch) {window.initSearch(searchIndex)};
if (typeof exports !== 'undefined') {exports.searchIndex = searchIndex};
