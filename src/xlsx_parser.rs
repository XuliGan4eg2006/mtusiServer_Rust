use calamine::{open_workbook, Error, Xlsx, Reader, RangeDeserializerBuilder};

pub fn get_groups_map() -> Vec<(String, i32, usize)> {
    let path = "D:\\Desktop_unz\\projects\\mtusiServer\\src\\test.xlsx";

    let mut excel: Xlsx<_> = open_workbook(path).unwrap();
    let range = match excel.worksheet_range("12.01-13.01"){
        Ok(r) => r,
        Err(e) => panic!("{:?}", e)
    };
    let mut group_coords = Vec::new();
    
    //идём по 4 ряду по каждому столбцу с 3 столбца до конца и выводим текст в консоль
    for (i, cell) in range[4][3..].iter().enumerate() {
        if cell.to_string().contains("-") {
            println!("{}", cell);
            //getting ordinal number of cell in range
            println!("Coordinate: (4, {})", i + 3);
            group_coords.push((cell.to_string(), 4, i + 3));
        }
    }
    
    group_coords
}