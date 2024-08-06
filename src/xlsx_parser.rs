use calamine::{open_workbook, Error, Xlsx, Reader, RangeDeserializerBuilder};
use serde_json::json;

const XLSX_PATH: &str = "D:\\Desktop_unz\\projects\\mtusiServer\\src\\test.xlsx";
pub fn get_groups_map() -> serde_json::Value {

    let mut excel: Xlsx<_> = open_workbook(XLSX_PATH).unwrap();
    let range = match excel.worksheet_range("12.01-13.01"){
        Ok(r) => r,
        Err(e) => panic!("{:?}", e)
    };
    let mut json_map = json!({});
    
    //идём по 4 ряду по каждому столбцу с 3 столбца до конца и выводим текст в консоль
    for (i, cell) in range[4][3..].iter().enumerate() {
        if cell.to_string().contains("-") {
            json_map[&cell.to_string().replace(" ", "")] = json!({"x": i + 3, "y": 4});
        }
    }
    json_map
}

pub fn get_timetable(group_map: serde_json::Value, group: &str) -> serde_json::Value {
    // getting group data from group_map by key group
    let group_coords = &group_map[group];

    if !&group_coords.is_null() {
        println!("Group found: {}", group);
    } else {
        println!("Group not found: {}", group);
    }

    let mut excel: Xlsx<_> = open_workbook(XLSX_PATH).unwrap();
    let range = match excel.worksheet_range("12.01-13.01"){
        Ok(r) => r,
        Err(e) => panic!("{:?}", e)
    };
    let mut json_timetable = json!({});
    let x_coord = group_coords["x"].as_u64().unwrap_or_else(|| panic!("Invalid x coordinate"));
    let y_coord = group_coords["y"].as_u64().unwrap_or_else(|| panic!("Invalid y coordinate"));

    let mut day_counter = 0;
    let mut classes_counter = 0;
    let mut classes:Vec<String> = vec![];

    for row in range.rows().skip(y_coord as usize + 3) { //y отсуп сверху (по вертикали), обрезаем его тут, чтоб не попадались лишние данные, а 3 добавляем чтоб обрезать ещё невидимые строки (это прикол таблицы)
        if classes_counter == 5 { //с нуля же начинаем
            json_timetable[day_counter.to_string()] = json!(classes); //day_counter.to_string() такова особенность приложения на мобиле, мб потом сделаю нормально

            day_counter += 1;
            classes_counter = 0;
            classes = vec![];

        }
        classes.push(row[x_coord as usize].to_string());
        classes_counter += 1;
        println!("{:?}",classes);
        //println!("{}", row[x_coord as usize]); //x это отступ по горизонтали (aka номер колонки группы), так как у нас каждая строка это массив,то и значение ячейки это её x оффсет
    }

    println!("{}", json_timetable);

    json_timetable
}