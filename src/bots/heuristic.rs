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
    map.insert((10, 2), 3116.2887703019346);
    map.insert((11, 2), 3122.3085594126323);
    map.insert((12, 2), 3047.345081707597);
    map.insert((13, 2), 3099.223916888474);
    map.insert((14, 2), 3182.2731144237323);
    map.insert((15, 2), 3214.7108366087255);
    map.insert((0, 3), 3063.025374081727);
    map.insert((1, 3), 3068.7994561636233);
    map.insert((2, 3), 3069.6197076845806);
    map.insert((3, 3), 3063.203603618352);
    map.insert((4, 3), 3048.0642844092717);
    map.insert((5, 3), 3034.0420289629624);
    map.insert((6, 3), 3021.019215006959);
    map.insert((7, 3), 3028.6616915215172);
    map.insert((8, 3), 3015.082091898981);
    map.insert((9, 3), 3019.862055044706);
    map.insert((10, 3), 3031.0081021217975);
    map.insert((11, 3), 3070.1901536595274);
    map.insert((12, 3), 3104.8852940101065);
    map.insert((13, 3), 3032.208786924165);
    map.insert((14, 3), 2932.3288353609255);
    map.insert((0, 4), 2808.89036039599);
    map.insert((1, 4), 2838.5732110606796);
    map.insert((2, 4), 2859.558136364507);
    map.insert((3, 4), 2865.7968866021615);
    map.insert((4, 4), 2868.0991106486927);
    map.insert((5, 4), 2868.5613685974145);
    map.insert((6, 4), 2915.366530080146);
    map.insert((7, 4), 2945.7700080726318);
    map.insert((8, 4), 2969.636506078993);
    map.insert((9, 4), 3007.246067594259);
    map.insert((10, 4), 3010.4782800385033);
    map.insert((11, 4), 3000.7147721183783);
    map.insert((12, 4), 3001.2781990771086);
    map.insert((13, 4), 3020.888379664035);
    map.insert((0, 5), 2088.9237111115544);
    map.insert((1, 5), 2147.1211385695588);
    map.insert((2, 5), 2217.9237646106103);
    map.insert((3, 5), 2280.6450348307935);
    map.insert((4, 5), 2378.7187661888634);
    map.insert((5, 5), 2354.657452120526);
    map.insert((6, 5), 2464.144831587634);
    map.insert((7, 5), 2538.6495126148484);
    map.insert((8, 5), 2806.025781353723);
    map.insert((9, 5), 2701.510589394494);
    map.insert((10, 5), 2655.759113006457);
    map.insert((11, 5), 2644.951512401021);
    map.insert((12, 5), 1976.9374422316691);
    map.insert((13, 5), 1839.8732081821986);
    map.insert((0, 6), 2212.4509605294147);
    map.insert((1, 6), 2232.964004582813);
    map.insert((2, 6), 2243.2615766120925);
    map.insert((3, 6), 2249.358271192578);
    map.insert((4, 6), 2255.7605994362884);
    map.insert((5, 6), 2260.3637785858164);
    map.insert((6, 6), 2261.443902201299);
    map.insert((7, 6), 2251.9473218076055);
    map.insert((8, 6), 2202.694135481581);
    map.insert((9, 6), 2084.708801627474);
    map.insert((10, 6), 1971.8402052407216);
    map.insert((11, 6), 2099.0709807661747);
    map.insert((12, 6), 2298.794189453125);
    map.insert((0, 7), 1095.8994803099276);
    map.insert((1, 7), 1031.2676256228744);
    map.insert((2, 7), 1233.8897725416261);
    map.insert((3, 7), 1288.2728400458664);
    map.insert((4, 7), 1353.253268887028);
    map.insert((5, 7), 1432.267674058248);
    map.insert((6, 7), 1607.9439983310128);
    map.insert((7, 7), 2394.7376788100814);
    map.insert((8, 7), 2404.7207729240868);
    map.insert((9, 7), 2388.0496335453736);
    map.insert((10, 7), 2376.3505267154173);
    map.insert((0, 8), 1179.47517034642);
    map.insert((1, 8), 1243.3977262185715);
    map.insert((2, 8), 1291.630910089711);
    map.insert((3, 8), 1302.9238838933109);
    map.insert((4, 8), 1316.0900845028248);
    map.insert((5, 8), 1310.4338729962612);
    map.insert((6, 8), 1311.7281078101312);
    map.insert((7, 8), 1328.8533736226354);
    map.insert((8, 8), 822.8842211275979);
    map.insert((9, 8), 564.4767397992409);
    map.insert((10, 8), 1224.637397303512);
    map.insert((11, 8), 1398.126708984375);
    map.insert((0, 9), 900.4807920794918);
    map.insert((1, 9), 1337.5753462260827);
    map.insert((2, 9), 1649.8049767603864);
    map.insert((3, 9), 1722.4083497829947);
    map.insert((4, 9), 1743.7367273184161);
    map.insert((5, 9), 1751.9862352618627);
    map.insert((6, 9), 1754.1341435996314);
    map.insert((7, 9), 1881.2397889571953);
    map.insert((8, 9), 1890.623609917044);
    map.insert((9, 9), 1568.112265236507);
    map.insert((10, 9), 1303.693157404029);
    map.insert((0, 10), 103.02655080187209);
    map.insert((1, 10), 109.14034585168685);
    map.insert((2, 10), 111.57540432477019);
    map.insert((3, 10), 112.12630947997722);
    map.insert((4, 10), 106.74258787860167);
    map.insert((5, 10), 118.04860344826089);
    map.insert((6, 10), 330.7022782710131);
    map.insert((7, 10), 510.4726553444748);
    map.insert((8, 10), 1003.2993334461993);
    map.insert((9, 10), 1016.5350534263582);
    map.insert((10, 10), 1103.4501426288573);
    map.insert((0, 11), 10.99386481457965);
    map.insert((1, 11), 10.894777656121262);
    map.insert((2, 11), 14.096805130591573);
    map.insert((3, 11), 16.732856770995);
    map.insert((4, 11), 25.68712525637572);
    map.insert((5, 11), 38.46363933054806);
    map.insert((6, 11), 205.00358337088943);
    map.insert((7, 11), 354.3236584635093);
    map.insert((8, 11), 409.14511139069805);
    map.insert((9, 11), 308.29683462621455);
    map.insert((10, 11), 550.1526912476444);
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
