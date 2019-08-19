var fs = require("fs")
var path = require("path")
var dictObj = require("./pinyin.dict").pinyin_dict
var os = require("os")

function transform () {
    var distpath = path.resolve(__dirname, "../src/dictdata.rs")
    var filewriter = null;
    if (fs.existsSync(distpath)) {
        fs.unlinkSync(distpath)
    } 
    filewriter = fs.createWriteStream(distpath)
    filewriter.write(`
use std::collections::HashMap;

pub fn get_dict_data() -> HashMap<u32, String>{
    let mut d = HashMap::new();
    
    `);

    let i = 0;
    for (let key in dictObj) {
        let val = dictObj[key];
        let line = `  d.insert(${key.charCodeAt(0)}, "${val}".to_string());`+os.EOL;
        filewriter.write(line);
    }
    filewriter.write(`
    d
}
    `)
    filewriter.close()

}


transform()
