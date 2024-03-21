var searchIndex = new Map(JSON.parse('[\
["tankerkoenig",{"doc":"tankerkoenig-rs","t":"PGPPPEPCNNQNNNNNCNNNNNOOOOFNNNNNNNNNCOCONNNNFNNNNNNNNNNNNNFNNNNNNNNNNNNNNEEEPPPPGPEGNNNNNNNNNNNNNNNNNNNNNNNNCCNNNNNNNNNNNNFFNNNNNNNNONNOOONNNNNNNNOONONNONNNNNNNNFFFFFFFNNNNNNNNNNNNNNOOONNNNNNNNNNNNNNNOOONNNNNNNOOOOOOOOONNNNNNNNNNNNNNNNNNNNNNOOOOOONNNNNNNOOOOOOOOOOOOOOOOOOOONNNNNNNOOOOOOONNNNNNNOOOOOOOOOOOONNNNNNNNNNNNNNNNNNNNNNNNNNNNO","n":["ClientConstruction","Error","HeaderConstruction","RequestError","ResponseParsingError","Tankerkoenig","UrlConstruction","api","borrow","borrow_mut","chunk_into_option_arrays","fmt","fmt","from","from","into","models","source","to_string","try_from","try_into","type_id","body","source","source","source","Tankerkoenig","borrow","borrow_mut","clone","clone_into","fmt","from","into","new","new_with_useragent","price","price","station","station","to_owned","try_from","try_into","type_id","PriceApi","borrow","borrow_mut","clone","clone_into","fetch","fetch_one","fmt","from","into","to_owned","try_from","try_into","type_id","StationApi","borrow","borrow_mut","clone","clone_into","fetch_by_fuel","fetch_details","fetch_near","fmt","from","into","to_owned","try_from","try_into","type_id","AreaFuelResponse","AreaNearResponse","DetailsResponse","Diesel","Distance","E10","E5","Fuel","Price","PriceResponse","Sort","borrow","borrow","borrow_mut","borrow_mut","clone","clone","clone_into","clone_into","cmp","cmp","eq","eq","fmt","fmt","fmt","fmt","from","from","from_str","from_str","into","into","partial_cmp","partial_cmp","price","station","to_owned","to_owned","to_string","to_string","try_from","try_from","try_from","try_from","try_into","try_into","type_id","type_id","PriceResponse","StationPrices","borrow","borrow","borrow_mut","borrow_mut","clone","clone","clone_into","clone_into","data","deserialize","deserialize","diesel","e10","e5","eq","eq","fmt","fmt","from","from","into","into","license","ok","partial_cmp","prices","serialize","serialize","status","to_owned","to_owned","try_from","try_from","try_into","try_into","type_id","type_id","AreaFuelResponse","AreaNearResponse","AreaStationFuel","DetailStation","DetailsResponse","NearStation","OpeningTimes","borrow","borrow","borrow","borrow","borrow","borrow","borrow","borrow_mut","borrow_mut","borrow_mut","borrow_mut","borrow_mut","borrow_mut","borrow_mut","brand","brand","brand","clone","clone","clone","clone","clone","clone","clone","clone_into","clone_into","clone_into","clone_into","clone_into","clone_into","clone_into","cmp","data","data","data","deserialize","deserialize","deserialize","deserialize","deserialize","deserialize","deserialize","diesel","diesel","dist","dist","e10","e10","e5","e5","end","eq","eq","eq","eq","eq","eq","eq","fmt","fmt","fmt","fmt","fmt","fmt","fmt","from","from","from","from","from","from","from","hash","house_number","house_number","house_number","id","id","id","into","into","into","into","into","into","into","is_open","is_open","is_open","lat","lat","lat","license","license","license","lng","lng","lng","name","name","name","ok","ok","ok","opening_times","overrides","partial_cmp","partial_cmp","partial_cmp","partial_cmp","partial_cmp","partial_cmp","partial_cmp","place","place","place","post_code","post_code","post_code","price","serialize","serialize","serialize","serialize","serialize","serialize","serialize","start","state","station","stations","stations","status","status","status","street","street","street","text","to_owned","to_owned","to_owned","to_owned","to_owned","to_owned","to_owned","try_from","try_from","try_from","try_from","try_from","try_from","try_from","try_into","try_into","try_into","try_into","try_into","try_into","try_into","type_id","type_id","type_id","type_id","type_id","type_id","type_id","whole_day"],"q":[[0,"tankerkoenig"],[22,"tankerkoenig::Error"],[26,"tankerkoenig::api"],[44,"tankerkoenig::api::price"],[58,"tankerkoenig::api::station"],[73,"tankerkoenig::models"],[122,"tankerkoenig::models::price"],[161,"tankerkoenig::models::station"],[336,"core::fmt"],[337,"core::fmt"],[338,"core::error"],[339,"core::option"],[340,"alloc::string"],[341,"core::result"],[342,"core::any"],[343,"core::convert"],[344,"core::fmt"],[345,"core::fmt"],[346,"serde::ser"],[347,"core::hash"]],"d":["Something went wrong during http client creation","Contains all possible errors of the crate","Something went wrong during header construction","Something went wrong during fetching of the tankerkoenig …","Something went wrong during the parsing of the …","","Failed to parse the request url","Module that contains the main Tankerkoenig struct that …","","","Macro to transform a vector into a vector of arrays with a …","","","","Returns the argument unchanged.","Calls <code>U::from(self)</code>.","Module contains Structs that describe all possible …","","","","","","Response body that could not be parsed","Error source","Error source","Error source","The main struct of the crate giving access to the …","","","","","","Returns the argument unchanged.","Calls <code>U::from(self)</code>.","Creates a new instance of the Tankerkoenig struct by …","Creates a new instance of the Tankerkoenig struct by …","Price module that gives access to the PriceApi struct","Provide access to all price related api resources","Station module that gives access to the StationApi struct","Provide access to all station related api resources","","","","","Struct that holds the current reqwest client of the …","","","","","Fetch the prices of all fuel types of the given station …","Fetch the prices of a single station","","Returns the argument unchanged.","Calls <code>U::from(self)</code>.","","","","","Struct that holds the current reqwest client of the …","","","","","Fetch all stations in a radius around the given …","Fetch informations about a certain station by id.","Fetch all stations near a given area with some basic …","","Returns the argument unchanged.","Calls <code>U::from(self)</code>.","","","","","","","","Diesel and serialized to <code>diesel</code>","Sort by distance from near to far and will serialize to …","Super90 or E10 and serialized to <code>e10</code>","Super95 or E5 and serialized to <code>e5</code>","Enum of all supported fuel types","Sort by price from lowest to highest price and will …","","Enum of supported sorting logic","","","","","","","","","","","","","","","","","Returns the argument unchanged.","Returns the argument unchanged.","","","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","","","This module contains rust structs that describe the …","This module contains rust structs that describe the …","","","","","","","","","","","","","Response of the tankerkoenig api mapped to a rust struct …","Fuel prices of a station. If one field is <code>Noone</code> the …","","","","","","","","","Data licence","","","Price for diesel","Price for E10","Price for E5","","","","","Returns the argument unchanged.","Returns the argument unchanged.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Data licence","Request status","","Fuel prices of requested stations","","","Station open or closed","","","","","","","","","Response of the tankerkoenig API for a request for fuel in …","Response of the tankerkoenig API for a request of fuel …","Information about the fuel of a station in the area, …","Detailed information of a fuel station, returned by the …","Response of the tankerkoenig API for a request of detailed …","Information of a fuel station in the area returned by the …","Opening times for a fuel station","","","","","","","","","","","","","","","Brand of the station (e.g. JET)","Brand of the station (e.g. JET)","Brand of the station (e.g. JET)","","","","","","","","","","","","","","","","Data source","Data source","Data source","","","","","","","","Diesel price","Diesel price","Distance from search point","Distance of the station","E10 price","E10 price","E5 price","E5 price","Closing time","","","","","","","","","","","","","","","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","","Street number","Street number","Street number","Station id","Station id","Station id","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Open or Closed","Open or closed","Currently open or closed","Latitude (geolocation)","Latitude (geolocation)","Latitude (geolocation)","Data licence","Data licence","Data licence","Longitude (geolocation)","Longitude (geolocation)","Longitude (geolocation)","Name of the station","Station name","Name of the station","Request status","Request status","Request status","Opening times of the station","Additional information","","","","","","","","Area of the station","Area of the station","Area of the station","Local post code","Local post code","Local post code","Price of the requested fuel","","","","","","","","Opening time","State of the station","Detailed fuel station information","Vector of fuel stations in the area","List of stations in the area with the requested fuel","Request status","Request status","Request status","Street where the station is located","Street where the station is located","Street where the station is located","Information about the scope of start end end","","","","","","","","","","","","","","","","","","","","","","","","","","","","","Open 24 hours"],"i":[1,0,1,1,1,0,1,0,1,1,0,1,1,1,1,1,0,1,1,1,1,1,36,37,38,39,0,10,10,10,10,10,10,10,10,10,0,10,0,10,10,10,10,10,0,14,14,14,14,14,14,14,14,14,14,14,14,14,0,18,18,18,18,18,18,18,18,18,18,18,18,18,18,0,0,0,20,21,20,20,0,21,0,0,20,21,20,21,20,21,20,21,20,21,20,21,20,20,21,21,20,21,20,21,20,21,20,21,0,0,20,21,20,21,20,20,21,21,20,21,20,21,0,0,16,28,16,28,16,28,16,28,16,16,28,28,28,28,16,28,16,28,16,28,16,28,16,16,28,16,16,28,28,16,28,16,28,16,28,16,28,0,0,0,0,0,0,0,24,31,22,32,23,33,34,24,31,22,32,23,33,34,31,32,33,24,31,22,32,23,33,34,24,31,22,32,23,33,34,34,24,22,23,24,31,22,32,23,33,34,31,33,31,32,31,33,31,33,34,24,31,22,32,23,33,34,24,31,22,32,23,33,34,24,31,22,32,23,33,34,34,31,32,33,31,32,33,24,31,22,32,23,33,34,31,32,33,31,32,33,24,22,23,31,32,33,31,32,33,24,22,23,33,33,24,31,22,32,23,33,34,31,32,33,31,32,33,32,24,31,22,32,23,33,34,34,33,23,24,22,24,22,23,31,32,33,34,24,31,22,32,23,33,34,24,31,22,32,23,33,34,24,31,22,32,23,33,34,24,31,22,32,23,33,34,33],"f":[0,0,0,0,0,0,0,0,[-1,-2,[],[]],[-1,-2,[],[]],0,[[1,2],3],[[1,2],3],[4,1],[-1,-1,[]],[-1,-2,[],[]],0,[1,[[6,[5]]]],[-1,7,[]],[-1,[[8,[-2]]],[],[]],[-1,[[8,[-2]]],[],[]],[-1,9,[]],0,0,0,0,0,[-1,-2,[],[]],[-1,-2,[],[]],[10,10],[[-1,-2],11,[],[]],[[10,2],3],[-1,-1,[]],[-1,-2,[],[]],[-1,[[8,[10,1]]],[[13,[12]]]],[[-1,-1],[[8,[10,1]]],[[13,[12]]]],0,0,0,0,[-1,-2,[],[]],[-1,[[8,[-2]]],[],[]],[-1,[[8,[-2]]],[],[]],[-1,9,[]],0,[-1,-2,[],[]],[-1,-2,[],[]],[14,14],[[-1,-2],11,[],[]],[[14,[15,[[6,[-1]]]]],[[8,[16,1]]],[[13,[12]],17]],[[14,-1],[[8,[16,1]]],[[13,[12]],17]],[[14,2],3],[-1,-1,[]],[-1,-2,[],[]],[-1,-2,[],[]],[-1,[[8,[-2]]],[],[]],[-1,[[8,[-2]]],[],[]],[-1,9,[]],0,[-1,-2,[],[]],[-1,-2,[],[]],[18,18],[[-1,-2],11,[],[]],[[18,19,19,19,20,21],[[8,[22,1]]]],[[18,-1],[[8,[23,1]]],[[13,[12]]]],[[18,19,19,19],[[8,[24,1]]]],[[18,2],3],[-1,-1,[]],[-1,-2,[],[]],[-1,-2,[],[]],[-1,[[8,[-2]]],[],[]],[-1,[[8,[-2]]],[],[]],[-1,9,[]],0,0,0,0,0,0,0,0,0,0,0,[-1,-2,[],[]],[-1,-2,[],[]],[-1,-2,[],[]],[-1,-2,[],[]],[20,20],[21,21],[[-1,-2],11,[],[]],[[-1,-2],11,[],[]],[[20,20],25],[[21,21],25],[[20,20],26],[[21,21],26],[[20,2],3],[[20,2],[[8,[11,27]]]],[[21,2],3],[[21,2],[[8,[11,27]]]],[-1,-1,[]],[-1,-1,[]],[12,[[8,[20,-1]]],[]],[12,[[8,[21,-1]]],[]],[-1,-2,[],[]],[-1,-2,[],[]],[[20,20],[[6,[25]]]],[[21,21],[[6,[25]]]],0,0,[-1,-2,[],[]],[-1,-2,[],[]],[-1,7,[]],[-1,7,[]],[-1,[[8,[-2]]],[],[]],[12,[[8,[20,-1]]],[]],[-1,[[8,[-2]]],[],[]],[12,[[8,[21,-1]]],[]],[-1,[[8,[-2]]],[],[]],[-1,[[8,[-2]]],[],[]],[-1,9,[]],[-1,9,[]],0,0,[-1,-2,[],[]],[-1,-2,[],[]],[-1,-2,[],[]],[-1,-2,[],[]],[16,16],[28,28],[[-1,-2],11,[],[]],[[-1,-2],11,[],[]],0,[-1,[[8,[16]]],29],[-1,[[8,[28]]],29],0,0,0,[[16,16],26],[[28,28],26],[[16,2],3],[[28,2],3],[-1,-1,[]],[-1,-1,[]],[-1,-2,[],[]],[-1,-2,[],[]],0,0,[[28,28],[[6,[25]]]],0,[[16,-1],8,30],[[28,-1],8,30],0,[-1,-2,[],[]],[-1,-2,[],[]],[-1,[[8,[-2]]],[],[]],[-1,[[8,[-2]]],[],[]],[-1,[[8,[-2]]],[],[]],[-1,[[8,[-2]]],[],[]],[-1,9,[]],[-1,9,[]],0,0,0,0,0,0,0,[-1,-2,[],[]],[-1,-2,[],[]],[-1,-2,[],[]],[-1,-2,[],[]],[-1,-2,[],[]],[-1,-2,[],[]],[-1,-2,[],[]],[-1,-2,[],[]],[-1,-2,[],[]],[-1,-2,[],[]],[-1,-2,[],[]],[-1,-2,[],[]],[-1,-2,[],[]],[-1,-2,[],[]],0,0,0,[24,24],[31,31],[22,22],[32,32],[23,23],[33,33],[34,34],[[-1,-2],11,[],[]],[[-1,-2],11,[],[]],[[-1,-2],11,[],[]],[[-1,-2],11,[],[]],[[-1,-2],11,[],[]],[[-1,-2],11,[],[]],[[-1,-2],11,[],[]],[[34,34],25],0,0,0,[-1,[[8,[24]]],29],[-1,[[8,[31]]],29],[-1,[[8,[22]]],29],[-1,[[8,[32]]],29],[-1,[[8,[23]]],29],[-1,[[8,[33]]],29],[-1,[[8,[34]]],29],0,0,0,0,0,0,0,0,0,[[24,24],26],[[31,31],26],[[22,22],26],[[32,32],26],[[23,23],26],[[33,33],26],[[34,34],26],[[24,2],3],[[31,2],3],[[22,2],3],[[32,2],3],[[23,2],3],[[33,2],3],[[34,2],3],[-1,-1,[]],[-1,-1,[]],[-1,-1,[]],[-1,-1,[]],[-1,-1,[]],[-1,-1,[]],[-1,-1,[]],[[34,-1],11,35],0,0,0,0,0,0,[-1,-2,[],[]],[-1,-2,[],[]],[-1,-2,[],[]],[-1,-2,[],[]],[-1,-2,[],[]],[-1,-2,[],[]],[-1,-2,[],[]],0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,[[24,24],[[6,[25]]]],[[31,31],[[6,[25]]]],[[22,22],[[6,[25]]]],[[32,32],[[6,[25]]]],[[23,23],[[6,[25]]]],[[33,33],[[6,[25]]]],[[34,34],[[6,[25]]]],0,0,0,0,0,0,0,[[24,-1],8,30],[[31,-1],8,30],[[22,-1],8,30],[[32,-1],8,30],[[23,-1],8,30],[[33,-1],8,30],[[34,-1],8,30],0,0,0,0,0,0,0,0,0,0,0,0,[-1,-2,[],[]],[-1,-2,[],[]],[-1,-2,[],[]],[-1,-2,[],[]],[-1,-2,[],[]],[-1,-2,[],[]],[-1,-2,[],[]],[-1,[[8,[-2]]],[],[]],[-1,[[8,[-2]]],[],[]],[-1,[[8,[-2]]],[],[]],[-1,[[8,[-2]]],[],[]],[-1,[[8,[-2]]],[],[]],[-1,[[8,[-2]]],[],[]],[-1,[[8,[-2]]],[],[]],[-1,[[8,[-2]]],[],[]],[-1,[[8,[-2]]],[],[]],[-1,[[8,[-2]]],[],[]],[-1,[[8,[-2]]],[],[]],[-1,[[8,[-2]]],[],[]],[-1,[[8,[-2]]],[],[]],[-1,[[8,[-2]]],[],[]],[-1,9,[]],[-1,9,[]],[-1,9,[]],[-1,9,[]],[-1,9,[]],[-1,9,[]],[-1,9,[]],0],"c":[],"p":[[6,"Error",0],[5,"Formatter",336],[8,"Result",336],[5,"InvalidHeaderValue",337],[10,"Error",338],[6,"Option",339],[5,"String",340],[6,"Result",341],[5,"TypeId",342],[5,"Tankerkoenig",26],[1,"tuple"],[1,"str"],[10,"AsRef",343],[5,"PriceApi",44],[1,"array"],[5,"PriceResponse",122],[10,"Display",336],[5,"StationApi",58],[1,"f64"],[6,"Fuel",73],[6,"Sort",73],[5,"AreaFuelResponse",161],[5,"DetailsResponse",161],[5,"AreaNearResponse",161],[6,"Ordering",344],[1,"bool"],[5,"Error",336],[5,"StationPrices",122],[10,"Deserializer",345],[10,"Serializer",346],[5,"NearStation",161],[5,"AreaStationFuel",161],[5,"DetailStation",161],[5,"OpeningTimes",161],[10,"Hasher",347],[15,"ResponseParsingError",22],[15,"RequestError",22],[15,"HeaderConstruction",22],[15,"ClientConstruction",22]],"b":[[11,"impl-Debug-for-TankerkoenigError"],[12,"impl-Display-for-TankerkoenigError"],[96,"impl-Debug-for-Fuel"],[97,"impl-Display-for-Fuel"],[98,"impl-Debug-for-Sort"],[99,"impl-Display-for-Sort"]]}]\
]'));
if (typeof exports !== 'undefined') exports.searchIndex = searchIndex;
else if (window.initSearch) window.initSearch(searchIndex);
