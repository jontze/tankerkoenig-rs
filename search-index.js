var searchIndex = JSON.parse('{\
"tankerkoenig":{"doc":"tankerkoenig-rs","t":"NENNNCNALLOLLLLLALLLLLLMMMMDLLLLLLLLLAMAMLLLLDLLLLLLLLLLLLLDLLLLLLLLLLLLLLCCCNNNNENCELLLLLLLLLLLLLLLLLLLLLLLLLLLLAALLLLLLLLLLLLDDLLLLLLLLMLLMMMLLLLLLLLMMLMLLMLLLLLLLLDDDDDDDLLLLLLLLLLLLLLMMMLLLLLLLLLLLLLLLMMMLLLLLLLMMMMMMMMMLLLLLLLLLLLLLLLLLLLLLLLLMMMMMMLLLLLLLMMMMMMMMMMMMMMMMMMMMLLLLLLLMMMMMMMLLLLLLLMMMMMMMMMMMMLLLLLLLLLLLLLLLLLLLLLLLLLLLLM","n":["ClientConstruction","Error","HeaderConstruction","RequestError","ResponseParsingError","Tankerkoenig","UrlConstruction","api","borrow","borrow_mut","chunk_into_option_arrays","fmt","fmt","from","from","into","models","provide","source","to_string","try_from","try_into","type_id","body","source","source","source","Tankerkoenig","borrow","borrow_mut","clone","clone_into","fmt","from","into","new","new_with_useragent","price","price","station","station","to_owned","try_from","try_into","type_id","PriceApi","borrow","borrow_mut","clone","clone_into","fetch","fetch_one","fmt","from","into","to_owned","try_from","try_into","type_id","StationApi","borrow","borrow_mut","clone","clone_into","fetch_by_fuel","fetch_details","fetch_near","fmt","from","into","to_owned","try_from","try_into","type_id","AreaFuelResponse","AreaNearResponse","DetailsResponse","Diesel","Distance","E10","E5","Fuel","Price","PriceResponse","Sort","borrow","borrow","borrow_mut","borrow_mut","clone","clone","clone_into","clone_into","cmp","cmp","eq","eq","equivalent","equivalent","equivalent","equivalent","fmt","fmt","fmt","fmt","from","from","from_str","from_str","into","into","partial_cmp","partial_cmp","price","station","to_owned","to_owned","to_string","to_string","try_from","try_from","try_from","try_from","try_into","try_into","type_id","type_id","PriceResponse","StationPrices","borrow","borrow","borrow_mut","borrow_mut","clone","clone","clone_into","clone_into","data","deserialize","deserialize","diesel","e10","e5","eq","eq","fmt","fmt","from","from","into","into","license","ok","partial_cmp","prices","serialize","serialize","status","to_owned","to_owned","try_from","try_from","try_into","try_into","type_id","type_id","AreaFuelResponse","AreaNearResponse","AreaStationFuel","DetailStation","DetailsResponse","NearStation","OpeningTimes","borrow","borrow","borrow","borrow","borrow","borrow","borrow","borrow_mut","borrow_mut","borrow_mut","borrow_mut","borrow_mut","borrow_mut","borrow_mut","brand","brand","brand","clone","clone","clone","clone","clone","clone","clone","clone_into","clone_into","clone_into","clone_into","clone_into","clone_into","clone_into","cmp","data","data","data","deserialize","deserialize","deserialize","deserialize","deserialize","deserialize","deserialize","diesel","diesel","dist","dist","e10","e10","e5","e5","end","eq","eq","eq","eq","eq","eq","eq","equivalent","equivalent","fmt","fmt","fmt","fmt","fmt","fmt","fmt","from","from","from","from","from","from","from","hash","house_number","house_number","house_number","id","id","id","into","into","into","into","into","into","into","is_open","is_open","is_open","lat","lat","lat","license","license","license","lng","lng","lng","name","name","name","ok","ok","ok","opening_times","overrides","partial_cmp","partial_cmp","partial_cmp","partial_cmp","partial_cmp","partial_cmp","partial_cmp","place","place","place","post_code","post_code","post_code","price","serialize","serialize","serialize","serialize","serialize","serialize","serialize","start","state","station","stations","stations","status","status","status","street","street","street","text","to_owned","to_owned","to_owned","to_owned","to_owned","to_owned","to_owned","try_from","try_from","try_from","try_from","try_from","try_from","try_from","try_into","try_into","try_into","try_into","try_into","try_into","try_into","type_id","type_id","type_id","type_id","type_id","type_id","type_id","whole_day"],"q":[[0,"tankerkoenig"],[23,"tankerkoenig::Error"],[27,"tankerkoenig::api"],[45,"tankerkoenig::api::price"],[59,"tankerkoenig::api::station"],[74,"tankerkoenig::models"],[127,"tankerkoenig::models::price"],[166,"tankerkoenig::models::station"]],"d":["Something went wrong during http client creation","Contains all possible errors of the crate","Something went wrong during header construction","Something went wrong during fetching of the tankerkoenig …","Something went wrong during the parsing of the …","","Failed to parse the request url","Module that contains the main Tankerkoenig struct that …","","","Macro to transform a vector into a vector of arrays with a …","","","Returns the argument unchanged.","","Calls <code>U::from(self)</code>.","Module contains Structs that describe all possible …","","","","","","","Response body that could not be parsed","Error source","Error source","Error source","The main struct of the crate giving access to the …","","","","","","Returns the argument unchanged.","Calls <code>U::from(self)</code>.","Creates a new instance of the Tankerkoenig struct by …","Creates a new instance of the Tankerkoenig struct by …","Price module that gives access to the PriceApi struct","Provide access to all price related api resources","Station module that gives access to the StationApi struct","Provide access to all station related api resources","","","","","Struct that holds the current reqwest client of the …","","","","","Fetch the prices of all fuel types of the given station …","Fetch the prices of a single station","","Returns the argument unchanged.","Calls <code>U::from(self)</code>.","","","","","Struct that holds the current reqwest client of the …","","","","","Fetch all stations in a radius around the given …","Fetch informations about a certain station by id.","Fetch all stations near a given area with some basic …","","Returns the argument unchanged.","Calls <code>U::from(self)</code>.","","","","","","","","Diesel and serialized to <code>diesel</code>","Sort by distance from near to far and will serialize to …","Super90 or E10 and serialized to <code>e10</code>","Super95 or E5 and serialized to <code>e5</code>","Enum of all supported fuel types","Sort by price from lowest to highest price and will …","","Enum of supported sorting logic","","","","","","","","","","","","","","","","","","","","","Returns the argument unchanged.","Returns the argument unchanged.","","","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","","","This module contains rust structs that describe the …","This module contains rust structs that describe the …","","","","","","","","","","","","","Response of the tankerkoenig api mapped to a rust struct …","Fuel prices of a station. If one field is <code>Noone</code> the …","","","","","","","","","Data licence","","","Price for diesel","Price for E10","Price for E5","","","","","Returns the argument unchanged.","Returns the argument unchanged.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Data licence","Request status","","Fuel prices of requested stations","","","Station open or closed","","","","","","","","","Response of the tankerkoenig API for a request for fuel in …","Response of the tankerkoenig API for a request of fuel …","Information about the fuel of a station in the area, …","Detailed information of a fuel station, returned by the …","Response of the tankerkoenig API for a request of detailed …","Information of a fuel station in the area returned by the …","Opening times for a fuel station","","","","","","","","","","","","","","","Brand of the station (e.g. JET)","Brand of the station (e.g. JET)","Brand of the station (e.g. JET)","","","","","","","","","","","","","","","","Data source","Data source","Data source","","","","","","","","Diesel price","Diesel price","Distance from search point","Distance of the station","E10 price","E10 price","E5 price","E5 price","Closing time","","","","","","","","","","","","","","","","","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","","Street number","Street number","Street number","Station id","Station id","Station id","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Open or Closed","Open or closed","Currently open or closed","Latitude (geolocation)","Latitude (geolocation)","Latitude (geolocation)","Data licence","Data licence","Data licence","Longitude (geolocation)","Longitude (geolocation)","Longitude (geolocation)","Name of the station","Station name","Name of the station","Request status","Request status","Request status","Opening times of the station","Additional information","","","","","","","","Area of the station","Area of the station","Area of the station","Local post code","Local post code","Local post code","Price of the requested fuel","","","","","","","","Opening time","State of the station","Detailed fuel station information","Vector of fuel stations in the area","List of stations in the area with the requested fuel","Request status","Request status","Request status","Street where the station is located","Street where the station is located","Street where the station is located","Information about the scope of start end end","","","","","","","","","","","","","","","","","","","","","","","","","","","","","Open 24 hours"],"i":[1,0,1,1,1,0,1,0,1,1,0,1,1,1,1,1,0,1,1,1,1,1,1,36,37,38,39,0,11,11,11,11,11,11,11,11,11,0,11,0,11,11,11,11,11,0,14,14,14,14,14,14,14,14,14,14,14,14,14,0,18,18,18,18,18,18,18,18,18,18,18,18,18,18,0,0,0,20,21,20,20,0,21,0,0,20,21,20,21,20,21,20,21,20,21,20,21,20,20,21,21,20,20,21,21,20,21,20,21,20,21,20,21,0,0,20,21,20,21,20,20,21,21,20,21,20,21,0,0,17,28,17,28,17,28,17,28,17,17,28,28,28,28,17,28,17,28,17,28,17,28,17,17,28,17,17,28,28,17,28,17,28,17,28,17,28,0,0,0,0,0,0,0,24,31,22,32,23,33,34,24,31,22,32,23,33,34,31,32,33,24,31,22,32,23,33,34,24,31,22,32,23,33,34,34,24,22,23,24,31,22,32,23,33,34,31,33,31,32,31,33,31,33,34,24,31,22,32,23,33,34,34,34,24,31,22,32,23,33,34,24,31,22,32,23,33,34,34,31,32,33,31,32,33,24,31,22,32,23,33,34,31,32,33,31,32,33,24,22,23,31,32,33,31,32,33,24,22,23,33,33,24,31,22,32,23,33,34,31,32,33,31,32,33,32,24,31,22,32,23,33,34,34,33,23,24,22,24,22,23,31,32,33,34,24,31,22,32,23,33,34,24,31,22,32,23,33,34,24,31,22,32,23,33,34,24,31,22,32,23,33,34,33],"f":[0,0,0,0,0,0,0,0,[[]],[[]],0,[[1,2],3],[[1,2],3],[[]],[4,1],[[]],0,[5],[1,[[7,[6]]]],[[],8],[[],9],[[],9],[[],10],0,0,0,0,0,[[]],[[]],[11,11],[[]],[[11,2],3],[[]],[[]],[[[13,[12]]],[[9,[11,1]]]],[[[13,[12]],[13,[12]]],[[9,[11,1]]]],0,0,0,0,[[]],[[],9],[[],9],[[],10],0,[[]],[[]],[14,14],[[]],[[14,[16,[[7,[[0,[[13,[12]],15]]]]]]],[[9,[17,1]]]],[[14,[0,[[13,[12]],15]]],[[9,[17,1]]]],[[14,2],3],[[]],[[]],[[]],[[],9],[[],9],[[],10],0,[[]],[[]],[18,18],[[]],[[18,19,19,19,20,21],[[9,[22,1]]]],[[18,[13,[12]]],[[9,[23,1]]]],[[18,19,19,19],[[9,[24,1]]]],[[18,2],3],[[]],[[]],[[]],[[],9],[[],9],[[],10],0,0,0,0,0,0,0,0,0,0,0,[[]],[[]],[[]],[[]],[20,20],[21,21],[[]],[[]],[[20,20],25],[[21,21],25],[[20,20],26],[[21,21],26],[[],26],[[],26],[[],26],[[],26],[[20,2],[[9,[27]]]],[[20,2],3],[[21,2],3],[[21,2],[[9,[27]]]],[[]],[[]],[12,[[9,[20]]]],[12,[[9,[21]]]],[[]],[[]],[[20,20],[[7,[25]]]],[[21,21],[[7,[25]]]],0,0,[[]],[[]],[[],8],[[],8],[[],9],[12,[[9,[20]]]],[12,[[9,[21]]]],[[],9],[[],9],[[],9],[[],10],[[],10],0,0,[[]],[[]],[[]],[[]],[17,17],[28,28],[[]],[[]],0,[29,[[9,[17]]]],[29,[[9,[28]]]],0,0,0,[[17,17],26],[[28,28],26],[[17,2],3],[[28,2],3],[[]],[[]],[[]],[[]],0,0,[[28,28],[[7,[25]]]],0,[[17,30],9],[[28,30],9],0,[[]],[[]],[[],9],[[],9],[[],9],[[],9],[[],10],[[],10],0,0,0,0,0,0,0,[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],0,0,0,[24,24],[31,31],[22,22],[32,32],[23,23],[33,33],[34,34],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[34,34],25],0,0,0,[29,[[9,[24]]]],[29,[[9,[31]]]],[29,[[9,[22]]]],[29,[[9,[32]]]],[29,[[9,[23]]]],[29,[[9,[33]]]],[29,[[9,[34]]]],0,0,0,0,0,0,0,0,0,[[24,24],26],[[31,31],26],[[22,22],26],[[32,32],26],[[23,23],26],[[33,33],26],[[34,34],26],[[],26],[[],26],[[24,2],3],[[31,2],3],[[22,2],3],[[32,2],3],[[23,2],3],[[33,2],3],[[34,2],3],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[34,35]],0,0,0,0,0,0,[[]],[[]],[[]],[[]],[[]],[[]],[[]],0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,[[24,24],[[7,[25]]]],[[31,31],[[7,[25]]]],[[22,22],[[7,[25]]]],[[32,32],[[7,[25]]]],[[23,23],[[7,[25]]]],[[33,33],[[7,[25]]]],[[34,34],[[7,[25]]]],0,0,0,0,0,0,0,[[24,30],9],[[31,30],9],[[22,30],9],[[32,30],9],[[23,30],9],[[33,30],9],[[34,30],9],0,0,0,0,0,0,0,0,0,0,0,0,[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[],9],[[],9],[[],9],[[],9],[[],9],[[],9],[[],9],[[],9],[[],9],[[],9],[[],9],[[],9],[[],9],[[],9],[[],10],[[],10],[[],10],[[],10],[[],10],[[],10],[[],10],0],"c":[],"p":[[4,"Error"],[3,"Formatter"],[6,"Result"],[3,"InvalidHeaderValue"],[3,"Demand"],[8,"Error"],[4,"Option"],[3,"String"],[4,"Result"],[3,"TypeId"],[3,"Tankerkoenig"],[15,"str"],[8,"AsRef"],[3,"PriceApi"],[8,"Display"],[15,"array"],[3,"PriceResponse"],[3,"StationApi"],[15,"f64"],[4,"Fuel"],[4,"Sort"],[3,"AreaFuelResponse"],[3,"DetailsResponse"],[3,"AreaNearResponse"],[4,"Ordering"],[15,"bool"],[3,"Error"],[3,"StationPrices"],[8,"Deserializer"],[8,"Serializer"],[3,"NearStation"],[3,"AreaStationFuel"],[3,"DetailStation"],[3,"OpeningTimes"],[8,"Hasher"],[13,"ResponseParsingError"],[13,"RequestError"],[13,"HeaderConstruction"],[13,"ClientConstruction"]]}\
}');
if (typeof window !== 'undefined' && window.initSearch) {window.initSearch(searchIndex)};
if (typeof exports !== 'undefined') {exports.searchIndex = searchIndex};
