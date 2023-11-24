use std::collections::HashMap;

use lazy_static::lazy_static;

pub type PreprocessedBoard = (u32, u32);

pub fn generate_lookup() -> HashMap<PreprocessedBoard, f64> {
    let mut map = HashMap::new();

    map.insert((11, 1), 3469.7003173828125);
    map.insert((12, 1), 3610.103654191301);
    map.insert((13, 1), 3613.6753993890225);
    map.insert((14, 1), 3561.7130533854165);
    map.insert((15, 1), 3698.61376953125);
    map.insert((4, 2), 2414.547848791079);
    map.insert((5, 2), 2598.564676885849);
    map.insert((6, 2), 3101.829523945135);
    map.insert((7, 2), 3081.43973119993);
    map.insert((8, 2), 3062.7831637490453);
    map.insert((9, 2), 3023.053374395352);
    map.insert((10, 2), 3049.3586665966227);
    map.insert((11, 2), 3149.0752848306124);
    map.insert((12, 2), 3223.04791752535);
    map.insert((13, 2), 3397.069259290126);
    map.insert((14, 2), 3436.6684356876813);
    map.insert((15, 2), 3435.9177998636355);
    map.insert((0, 3), 3061.4932770283594);
    map.insert((1, 3), 2987.9680976038003);
    map.insert((2, 3), 3056.1877569005796);
    map.insert((3, 3), 3060.884099522807);
    map.insert((4, 3), 3084.014229002943);
    map.insert((5, 3), 3073.632004852651);
    map.insert((6, 3), 3061.535795214834);
    map.insert((7, 3), 3062.0583300872813);
    map.insert((8, 3), 3052.7185815111384);
    map.insert((9, 3), 3081.7697808785597);
    map.insert((10, 3), 3129.509506697261);
    map.insert((11, 3), 2904.5505819500354);
    map.insert((12, 3), 2929.3767031423313);
    map.insert((13, 3), 2949.957379756409);
    map.insert((14, 3), 3411.4591969118683);
    map.insert((0, 4), 2870.3060247596204);
    map.insert((1, 4), 2879.205994035568);
    map.insert((2, 4), 2878.3544614679245);
    map.insert((3, 4), 2878.094839671267);
    map.insert((4, 4), 2876.4630158395207);
    map.insert((5, 4), 2907.2174370259213);
    map.insert((6, 4), 2944.924777843086);
    map.insert((7, 4), 2988.602608842911);
    map.insert((8, 4), 2938.4689389632044);
    map.insert((9, 4), 3000.9887755366008);
    map.insert((10, 4), 2838.563394431671);
    map.insert((11, 4), 3100.3993022804707);
    map.insert((12, 4), 2691.38005653252);
    map.insert((13, 4), 2674.640870439648);
    map.insert((0, 5), 2333.738319083371);
    map.insert((1, 5), 2359.2318181150213);
    map.insert((2, 5), 2450.517553995208);
    map.insert((3, 5), 2526.7037692474296);
    map.insert((4, 5), 2617.250649209899);
    map.insert((5, 5), 2761.43001292627);
    map.insert((6, 5), 2704.7314428432524);
    map.insert((7, 5), 2538.834398353704);
    map.insert((8, 5), 2777.4487758256023);
    map.insert((9, 5), 2834.127772983388);
    map.insert((10, 5), 2853.2435969863036);
    map.insert((11, 5), 2443.2985575775833);
    map.insert((12, 5), 1976.9374422316691);
    map.insert((13, 5), 1839.8732081821986);
    map.insert((0, 6), 1914.4123522962998);
    map.insert((1, 6), 1903.069910128462);
    map.insert((2, 6), 1981.3906398828165);
    map.insert((3, 6), 2017.1151585568307);
    map.insert((4, 6), 2023.1169306830232);
    map.insert((5, 6), 2054.182960548346);
    map.insert((6, 6), 2075.0652730082616);
    map.insert((7, 6), 2107.0164079334663);
    map.insert((8, 6), 2020.6264097089654);
    map.insert((9, 6), 1969.0235575139225);
    map.insert((10, 6), 1971.8402052407216);
    map.insert((11, 6), 2099.0709807661747);
    map.insert((12, 6), 2298.794189453125);
    map.insert((0, 7), 681.1991932757038);
    map.insert((1, 7), 1054.5018966955442);
    map.insert((2, 7), 1288.1408451440118);
    map.insert((3, 7), 1336.3849245844228);
    map.insert((4, 7), 1355.5218876592905);
    map.insert((5, 7), 1512.410436391095);
    map.insert((6, 7), 1870.323492921461);
    map.insert((7, 7), 2045.5832298648688);
    map.insert((8, 7), 2171.937554994382);
    map.insert((9, 7), 2080.9647538722343);
    map.insert((10, 7), 1109.219687721946);
    map.insert((0, 8), 138.11392743340025);
    map.insert((1, 8), 154.96239500891835);
    map.insert((2, 8), 167.91735147049133);
    map.insert((3, 8), 172.70341897932886);
    map.insert((4, 8), 173.7843801565455);
    map.insert((5, 8), 197.0779747890087);
    map.insert((6, 8), 398.23653991759807);
    map.insert((7, 8), 528.8178120639184);
    map.insert((8, 8), 1433.1185390479088);
    map.insert((9, 8), 1702.6288457012581);
    map.insert((10, 8), 938.2472978071733);
    map.insert((11, 8), 1398.126708984375);
    map.insert((0, 9), 234.05158379360054);
    map.insert((1, 9), 684.8589586362181);
    map.insert((2, 9), 990.9523080026366);
    map.insert((3, 9), 1134.0933135496555);
    map.insert((4, 9), 1177.1984509686404);
    map.insert((5, 9), 1187.0330544927958);
    map.insert((6, 9), 1191.4052113822117);
    map.insert((7, 9), 1193.544973669005);
    map.insert((8, 9), 872.2805774576146);
    map.insert((9, 9), 1269.6122009912508);
    map.insert((10, 9), 1303.693157404029);
    map.insert((0, 10), 269.9609149962134);
    map.insert((1, 10), 1254.5890696486924);
    map.insert((2, 10), 1674.5352925819343);
    map.insert((3, 10), 1725.2167427251543);
    map.insert((4, 10), 1730.8934686999348);
    map.insert((5, 10), 1730.0387653700434);
    map.insert((6, 10), 1700.322343905785);
    map.insert((7, 10), 1630.8676212547657);
    map.insert((8, 10), 1234.2344593262526);
    map.insert((9, 10), 1479.2599641308302);
    map.insert((10, 10), 1248.7239970582543);
    map.insert((0, 11), 3.2423243490600497);
    map.insert((1, 11), 4.030208779006283);
    map.insert((2, 11), 6.226940041040755);
    map.insert((3, 11), 8.885789176173343);
    map.insert((4, 11), 19.632311484618203);
    map.insert((5, 11), 53.85890337713851);
    map.insert((6, 11), 133.6200434645888);
    map.insert((7, 11), 789.3640896809692);
    map.insert((8, 11), 837.3013120083383);
    map.insert((9, 11), 953.2221089491218);
    map.insert((10, 11), 1858.3662115074035);
    map.insert((0, 12), 80.17203224999271);
    map.insert((1, 12), 92.79287141161413);
    map.insert((2, 12), 82.86086660698443);
    map.insert((3, 12), 83.57377704955591);
    map.insert((4, 12), 75.82602119495184);
    map.insert((5, 12), 51.46136546364257);
    map.insert((6, 12), 47.455543316518195);
    map.insert((7, 12), 43.69681144563725);
    map.insert((8, 12), 26.85899097818347);
    map.insert((9, 12), 25.124716332382043);

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
        .unwrap_or(2_usize.pow(empty_count + 1) as f64)
}
