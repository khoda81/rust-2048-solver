use std::collections::HashMap;

use lazy_static::lazy_static;

pub type PreprocessedBoard = (u32, u32);

pub fn generate_lookup() -> HashMap<PreprocessedBoard, f64> {
    let mut map = HashMap::new();

    map.insert((0, 3), 58.60468040361211);
    map.insert((0, 4), 2062.834716796875);
    map.insert((0, 5), 1716.0806884765625);
    map.insert((0, 6), 1374.5482788085938);
    map.insert((0, 7), 605.5163685725286);
    map.insert((0, 8), 623.260009765625);
    map.insert((0, 9), 858.7399406433105);
    map.insert((0, 10), 65.746138215065);
    map.insert((0, 11), 83.33585357666016);
    map.insert((0, 12), 80.17203224999271);
    map.insert((1, 3), 78.13757200063645);
    map.insert((1, 4), 2067.0262044270835);
    map.insert((1, 5), 1723.1923828125);
    map.insert((1, 6), 1122.060546875);
    map.insert((1, 7), 543.3872623443604);
    map.insert((1, 8), 684.318577448527);
    map.insert((1, 9), 802.621885172526);
    map.insert((1, 10), 67.2357177734375);
    map.insert((1, 11), 127.21686953306198);
    map.insert((1, 12), 92.79287141161413);
    map.insert((2, 3), 1979.9863413864073);
    map.insert((2, 4), 2440.752197265625);
    map.insert((2, 5), 2056.610595703125);
    map.insert((2, 6), 1620.7033004760742);
    map.insert((2, 7), 800.1751098632813);
    map.insert((2, 8), 709.5849874130148);
    map.insert((2, 9), 994.6860961914063);
    map.insert((2, 10), 422.91356658935547);
    map.insert((2, 11), 218.22855154673258);
    map.insert((2, 12), 82.86086660698443);
    map.insert((3, 3), 2188.770373009684);
    map.insert((3, 4), 2657.75537109375);
    map.insert((3, 5), 1979.5580947204205);
    map.insert((3, 6), 1730.2189331054688);
    map.insert((3, 7), 989.6530529203869);
    map.insert((3, 8), 776.1714023323947);
    map.insert((3, 9), 1003.7958068847656);
    map.insert((3, 10), 590.2786254882813);
    map.insert((3, 11), 349.8266094147213);
    map.insert((3, 12), 83.57377704955591);
    map.insert((4, 2), 2414.547848791079);
    map.insert((4, 3), 3048.199589938444);
    map.insert((4, 4), 2647.6817220052085);
    map.insert((4, 5), 1986.5741930509869);
    map.insert((4, 6), 1697.7325439453125);
    map.insert((4, 7), 1014.9598083496094);
    map.insert((4, 8), 821.1128506595142);
    map.insert((4, 9), 999.0684988839286);
    map.insert((4, 10), 836.4800109863281);
    map.insert((4, 11), 348.5660095214844);
    map.insert((4, 12), 75.82602119495184);
    map.insert((5, 2), 2598.564676885849);
    map.insert((5, 3), 3175.2615687402154);
    map.insert((5, 4), 2651.650634765625);
    map.insert((5, 5), 2001.1061045328777);
    map.insert((5, 6), 1696.058349609375);
    map.insert((5, 7), 1012.3077448064631);
    map.insert((5, 8), 827.5258822354403);
    map.insert((5, 9), 1002.4301452636719);
    map.insert((5, 10), 830.0028686523438);
    map.insert((5, 11), 400.2858560583094);
    map.insert((5, 12), 51.46136546364257);
    map.insert((6, 2), 2581.242912936484);
    map.insert((6, 3), 3179.8556870023704);
    map.insert((6, 4), 2581.498837678329);
    map.insert((6, 5), 1888.8505539190573);
    map.insert((6, 6), 1685.5402205757473);
    map.insert((6, 7), 1014.2919921875);
    map.insert((6, 8), 819.1802031418373);
    map.insert((6, 9), 967.4093017578125);
    map.insert((6, 10), 824.9806068821957);
    map.insert((6, 11), 377.03544423277947);
    map.insert((6, 12), 47.455543316518195);
    map.insert((7, 2), 2925.3310526636465);
    map.insert((7, 3), 3194.94189453125);
    map.insert((7, 4), 2551.7485018643465);
    map.insert((7, 5), 1869.7481959751674);
    map.insert((7, 6), 1546.990101643525);
    map.insert((7, 7), 1012.1225852966309);
    map.insert((7, 8), 706.2539820177801);
    map.insert((7, 9), 810.6211954752604);
    map.insert((7, 10), 824.2447808957568);
    map.insert((7, 11), 386.327328931104);
    map.insert((7, 12), 43.69681144563725);
    map.insert((8, 2), 2978.395438461806);
    map.insert((8, 3), 3220.0979682074653);
    map.insert((8, 4), 2526.354918077257);
    map.insert((8, 5), 1735.3883495796017);
    map.insert((8, 6), 1653.1412828233506);
    map.insert((8, 7), 1010.0424974229601);
    map.insert((8, 8), 639.3767047458225);
    map.insert((8, 9), 809.1304931640625);
    map.insert((8, 10), 770.8073174062401);
    map.insert((8, 11), 384.33600849750616);
    map.insert((8, 12), 26.85899097818347);
    map.insert((9, 2), 3072.475364697188);
    map.insert((9, 3), 3399.9693603515625);
    map.insert((9, 4), 2508.198681640625);
    map.insert((9, 5), 1650.5851196289063);
    map.insert((9, 6), 1582.7266379220146);
    map.insert((9, 7), 997.2023113250732);
    map.insert((9, 8), 626.1317942301432);
    map.insert((9, 9), 813.7362670898438);
    map.insert((9, 10), 697.8438397067825);
    map.insert((9, 11), 380.45971450805666);
    map.insert((9, 12), 25.124716332382043);
    map.insert((10, 2), 3321.702057615389);
    map.insert((10, 3), 3342.054443359375);
    map.insert((10, 4), 2512.268798828125);
    map.insert((10, 5), 1670.8416323674376);
    map.insert((10, 6), 1772.4802059764886);
    map.insert((10, 7), 1109.219687721946);
    map.insert((10, 8), 1200.3842942494564);
    map.insert((10, 9), 812.3741092354911);
    map.insert((10, 10), 788.4899481571082);
    map.insert((10, 11), 382.628305608576);
    map.insert((11, 1), 5085.774088541667);
    map.insert((11, 2), 3616.363381581764);
    map.insert((11, 3), 2956.8121337890625);
    map.insert((11, 4), 2516.951416015625);
    map.insert((11, 5), 1746.5060098232818);
    map.insert((11, 6), 2099.0709807661747);
    map.insert((11, 8), 1398.126708984375);
    map.insert((12, 1), 5036.91974609375);
    map.insert((12, 2), 3741.700439453125);
    map.insert((12, 3), 3007.361796476403);
    map.insert((12, 4), 2672.1209050958805);
    map.insert((12, 5), 1862.9499143029227);
    map.insert((12, 6), 2298.794189453125);
    map.insert((13, 1), 4981.615097045898);
    map.insert((13, 2), 4414.72297668457);
    map.insert((13, 3), 3128.3585597446986);
    map.insert((13, 4), 2440.780939600489);
    map.insert((13, 5), 1839.8732081821986);
    map.insert((14, 1), 5120.7568359375);
    map.insert((14, 2), 5073.520740597747);
    map.insert((14, 3), 5184.921769205729);
    map.insert((15, 1), 4935.69677734375);

    map
}

lazy_static! {
    static ref PRE_LOOKUP: HashMap<PreprocessedBoard, f64> = generate_lookup();
}

pub fn get_lookup() -> &'static HashMap<PreprocessedBoard, f64> {
    &PRE_LOOKUP
}

pub fn heuristic(preprocessed_board: PreprocessedBoard) -> f64 {
    let (empty_count, _max_cell) = preprocessed_board;

    // TODO: stop creating the hashmap every single time
    let lookup = get_lookup();
    let pre_lookup = lookup.get(&preprocessed_board);

    let empty_count_lookup = [
        15.82, 35.14, 752.49, 633.58, 1909.69, 3259.14, 3320.45, 3356.29, 3388.47, 3388.15,
        3446.54, 3541.35, 4071.11, 4961.21, 7341.16, 9085.73,
    ]
    .get(empty_count as usize);

    pre_lookup
        .or(empty_count_lookup)
        .cloned()
        .unwrap_or(2_usize.pow(empty_count + 1) as f64);

    2_usize.pow(empty_count + 1) as f64
}
