var searchIndex = JSON.parse('{\
"notifier":{"doc":"Main","t":"AAFAAFFFFFFDLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLMLLLLLLLLLLLLLLLLLLLLLLLLLLLLMMLLLLLLLLFF","n":["get","helpers","main","message","post","rocket","channel","general","health","retrieve_message","store_message","Message","attr","bg","black","blink","blue","bold","borrow","borrow_mut","bright","bright_black","bright_blue","bright_cyan","bright_green","bright_magenta","bright_red","bright_white","bright_yellow","clear","conceal","cyan","deserialize","dim","fg","fixed","from","green","into","into_collection","invert","italic","linger","magenta","mapped","mask","message","new","on_black","on_blue","on_bright","on_bright_black","on_bright_blue","on_bright_cyan","on_bright_green","on_bright_magenta","on_bright_red","on_bright_white","on_bright_yellow","on_cyan","on_fixed","on_green","on_magenta","on_primary","on_red","on_rgb","on_white","on_yellow","primary","quirk","rapid_blink","red","rgb","serialize","strike","style","title","try_from","try_into","type_id","underline","whenever","white","wrap","yellow","channel","general"],"q":[[0,"notifier"],[6,"notifier::get"],[9,"notifier::helpers"],[11,"notifier::message"],[85,"notifier::post"]],"d":["This module contains the GET routes for the API.","","","","This module contains the POST routes for the API.","This function starts the application and mounts the routes.","This route is for any specified channel.","This route is for the general channel.","This route is for the health check.","","","","Enables the styling <code>Attribute</code> <code>value</code>.","Returns a styled value derived from <code>self</code> with the …","Returns <code>self</code> with the <code>fg()</code> set to <code>Color::Black</code>.","Returns <code>self</code> with the <code>attr()</code> set to <code>Attribute::Blink</code>.","Returns <code>self</code> with the <code>fg()</code> set to <code>Color::Blue</code>.","Returns <code>self</code> with the <code>attr()</code> set to <code>Attribute::Bold</code>.","","","Returns <code>self</code> with the <code>quirk()</code> set to <code>Quirk::Bright</code>.","Returns <code>self</code> with the <code>fg()</code> set to <code>Color::BrightBlack</code>.","Returns <code>self</code> with the <code>fg()</code> set to <code>Color::BrightBlue</code>.","Returns <code>self</code> with the <code>fg()</code> set to <code>Color::BrightCyan</code>.","Returns <code>self</code> with the <code>fg()</code> set to <code>Color::BrightGreen</code>.","Returns <code>self</code> with the <code>fg()</code> set to <code>Color::BrightMagenta</code>.","Returns <code>self</code> with the <code>fg()</code> set to <code>Color::BrightRed</code>.","Returns <code>self</code> with the <code>fg()</code> set to <code>Color::BrightWhite</code>.","Returns <code>self</code> with the <code>fg()</code> set to <code>Color::BrightYellow</code>.","Returns <code>self</code> with the <code>quirk()</code> set to <code>Quirk::Clear</code>.","Returns <code>self</code> with the <code>attr()</code> set to <code>Attribute::Conceal</code>.","Returns <code>self</code> with the <code>fg()</code> set to <code>Color::Cyan</code>.","","Returns <code>self</code> with the <code>attr()</code> set to <code>Attribute::Dim</code>.","Returns a styled value derived from <code>self</code> with the …","Returns <code>self</code> with the <code>fg()</code> set to <code>Color::Fixed</code>.","Returns the argument unchanged.","Returns <code>self</code> with the <code>fg()</code> set to <code>Color::Green</code>.","Calls <code>U::from(self)</code>.","","Returns <code>self</code> with the <code>attr()</code> set to <code>Attribute::Invert</code>.","Returns <code>self</code> with the <code>attr()</code> set to <code>Attribute::Italic</code>.","Returns <code>self</code> with the <code>quirk()</code> set to <code>Quirk::Linger</code>.","Returns <code>self</code> with the <code>fg()</code> set to <code>Color::Magenta</code>.","","Returns <code>self</code> with the <code>quirk()</code> set to <code>Quirk::Mask</code>.","","","Returns <code>self</code> with the <code>bg()</code> set to <code>Color::Black</code>.","Returns <code>self</code> with the <code>bg()</code> set to <code>Color::Blue</code>.","Returns <code>self</code> with the <code>quirk()</code> set to <code>Quirk::OnBright</code>.","Returns <code>self</code> with the <code>bg()</code> set to <code>Color::BrightBlack</code>.","Returns <code>self</code> with the <code>bg()</code> set to <code>Color::BrightBlue</code>.","Returns <code>self</code> with the <code>bg()</code> set to <code>Color::BrightCyan</code>.","Returns <code>self</code> with the <code>bg()</code> set to <code>Color::BrightGreen</code>.","Returns <code>self</code> with the <code>bg()</code> set to <code>Color::BrightMagenta</code>.","Returns <code>self</code> with the <code>bg()</code> set to <code>Color::BrightRed</code>.","Returns <code>self</code> with the <code>bg()</code> set to <code>Color::BrightWhite</code>.","Returns <code>self</code> with the <code>bg()</code> set to <code>Color::BrightYellow</code>.","Returns <code>self</code> with the <code>bg()</code> set to <code>Color::Cyan</code>.","Returns <code>self</code> with the <code>bg()</code> set to <code>Color::Fixed</code>.","Returns <code>self</code> with the <code>bg()</code> set to <code>Color::Green</code>.","Returns <code>self</code> with the <code>bg()</code> set to <code>Color::Magenta</code>.","Returns <code>self</code> with the <code>bg()</code> set to <code>Color::Primary</code>.","Returns <code>self</code> with the <code>bg()</code> set to <code>Color::Red</code>.","Returns <code>self</code> with the <code>bg()</code> set to <code>Color::Rgb</code>.","Returns <code>self</code> with the <code>bg()</code> set to <code>Color::White</code>.","Returns <code>self</code> with the <code>bg()</code> set to <code>Color::Yellow</code>.","Returns <code>self</code> with the <code>fg()</code> set to <code>Color::Primary</code>.","Enables the <code>yansi</code> <code>Quirk</code> <code>value</code>.","Returns <code>self</code> with the <code>attr()</code> set to <code>Attribute::RapidBlink</code>.","Returns <code>self</code> with the <code>fg()</code> set to <code>Color::Red</code>.","Returns <code>self</code> with the <code>fg()</code> set to <code>Color::Rgb</code>.","","Returns <code>self</code> with the <code>attr()</code> set to <code>Attribute::Strike</code>.","","","","","","Returns <code>self</code> with the <code>attr()</code> set to <code>Attribute::Underline</code>.","Conditionally enable styling based on whether the <code>Condition</code>…","Returns <code>self</code> with the <code>fg()</code> set to <code>Color::White</code>.","Returns <code>self</code> with the <code>quirk()</code> set to <code>Quirk::Wrap</code>.","Returns <code>self</code> with the <code>fg()</code> set to <code>Color::Yellow</code>.","This route is for any specified channel.","This route is for the general channel."],"i":[0,0,0,0,0,0,0,0,0,0,0,0,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,0,0],"f":[0,0,[[]],0,0,[[],[[2,[1]]]],[3,[[7,[[5,[4]],6]]]],[[],[[7,[[5,[4]],6]]]],[[],[[5,[4]]]],[8,[[7,[[5,[4]],6]]]],[[8,4],6],0,[9,10],[11,10],[[],10],[[],10],[[],10],[[],10],[[]],[[]],[[],10],[[],10],[[],10],[[],10],[[],10],[[],10],[[],10],[[],10],[[],10],[[],10],[[],10],[[],10],[12,[[7,[4]]]],[[],10],[11,10],[13,10],[[]],[[],10],[[]],[[],[[15,[14]]]],[[],10],[[],10],[[],10],[[],10],[16,[[15,[14]]]],[[],10],0,[[3,3,3],[[5,[4]]]],[[],10],[[],10],[[],10],[[],10],[[],10],[[],10],[[],10],[[],10],[[],10],[[],10],[[],10],[[],10],[13,10],[[],10],[[],10],[[],10],[[],10],[[13,13,13],10],[[],10],[[],10],[[],10],[17,10],[[],10],[[],10],[[13,13,13],10],[[4,18],7],[[],10],0,0,[[],7],[[],7],[[],19],[[],10],[20,10],[[],10],[[],10],[[],10],[[3,[5,[4]]],6],[[[5,[4]]],6]],"c":[],"p":[[4,"Build"],[3,"Rocket"],[3,"String"],[3,"Message"],[3,"Json"],[3,"Status"],[4,"Result"],[15,"str"],[4,"Attribute"],[3,"Painted"],[4,"Color"],[8,"Deserializer"],[15,"u8"],[8,"Array"],[3,"SmallVec"],[8,"FnMut"],[4,"Quirk"],[8,"Serializer"],[3,"TypeId"],[3,"Condition"]]}\
}');
if (typeof window !== 'undefined' && window.initSearch) {window.initSearch(searchIndex)};
if (typeof exports !== 'undefined') {exports.searchIndex = searchIndex};
