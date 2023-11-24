use std::collections::HashMap;

use lazy_static::lazy_static;

pub type PreprocessedBoard = (u32, u32);

pub fn generate_lookup() -> HashMap<PreprocessedBoard, f64> {
    let mut map = HashMap::new();
    map.insert((0, 3), 58.60468040361211);
    map.insert((0, 4), 98.78200511190586);
    map.insert((0, 5), 114.35688882776908);
    map.insert((0, 6), 710.2698291220651);
    map.insert((0, 7), 476.1168285740245);
    map.insert((0, 8), 297.00176677908235);
    map.insert((0, 9), 519.7970793284996);
    map.insert((0, 10), 1792.5957223781415);
    map.insert((0, 11), 174.54889676889584);
    map.insert((0, 12), 80.17203224999271);
    map.insert((1, 3), 78.13757200063645);
    map.insert((1, 4), 96.83730226991726);
    map.insert((1, 5), 117.10243033850661);
    map.insert((1, 6), 1810.4976369432306);
    map.insert((1, 7), 480.1403004033035);
    map.insert((1, 8), 348.2624912585843);
    map.insert((1, 9), 499.0263081532593);
    map.insert((1, 10), 598.5319074593805);
    map.insert((1, 11), 181.31909206846365);
    map.insert((1, 12), 92.79287141161413);
    map.insert((2, 3), 1979.9863413864073);
    map.insert((2, 4), 1961.5951609913116);
    map.insert((2, 5), 133.97924108549486);
    map.insert((2, 6), 2854.500527369244);
    map.insert((2, 7), 482.081220132172);
    map.insert((2, 8), 381.2055956431272);
    map.insert((2, 9), 506.7555772249735);
    map.insert((2, 10), 1770.0630468434256);
    map.insert((2, 11), 181.29056221939146);
    map.insert((2, 12), 82.86086660698443);
    map.insert((3, 3), 2188.770373009684);
    map.insert((3, 4), 2078.910082042182);
    map.insert((3, 5), 102.78475738349816);
    map.insert((3, 6), 2864.758811695918);
    map.insert((3, 7), 498.34360749018555);
    map.insert((3, 8), 381.1866727469817);
    map.insert((3, 9), 545.8031624011111);
    map.insert((3, 10), 1768.5794889951103);
    map.insert((3, 11), 173.39696309344134);
    map.insert((3, 12), 83.57377704955591);
    map.insert((4, 2), 2414.547848791079);
    map.insert((4, 3), 2245.2840634335585);
    map.insert((4, 4), 2088.560920619206);
    map.insert((4, 5), 1461.8127675764702);
    map.insert((4, 6), 2809.3268267636868);
    map.insert((4, 7), 486.5273317379003);
    map.insert((4, 8), 412.48048032252876);
    map.insert((4, 9), 547.1206358263103);
    map.insert((4, 10), 1881.650993506125);
    map.insert((4, 11), 176.75135576065037);
    map.insert((4, 12), 75.82602119495184);
    map.insert((5, 2), 2598.564676885849);
    map.insert((5, 3), 2283.1962377901764);
    map.insert((5, 4), 2089.7464978087137);
    map.insert((5, 5), 1093.4684653870586);
    map.insert((5, 6), 2803.6990662023836);
    map.insert((5, 7), 477.014298995085);
    map.insert((5, 8), 482.51909953383193);
    map.insert((5, 9), 572.6520643324939);
    map.insert((5, 10), 1876.6965183680352);
    map.insert((5, 11), 181.36623861074207);
    map.insert((5, 12), 51.46136546364257);
    map.insert((6, 2), 2605.969730148572);
    map.insert((6, 3), 2352.696625238423);
    map.insert((6, 4), 2090.211705829966);
    map.insert((6, 5), 1340.0959806936978);
    map.insert((6, 6), 2823.749783869496);
    map.insert((6, 7), 466.16391986783316);
    map.insert((6, 8), 335.5046840749344);
    map.insert((6, 9), 562.7630221623006);
    map.insert((6, 10), 1821.8426431206526);
    map.insert((6, 11), 177.96065952517714);
    map.insert((6, 12), 47.455543316518195);
    map.insert((7, 2), 2615.252339235343);
    map.insert((7, 3), 2470.8570052633845);
    map.insert((7, 4), 2078.0583851973884);
    map.insert((7, 5), 1345.022274990999);
    map.insert((7, 6), 2761.7193044405726);
    map.insert((7, 7), 463.30139690367423);
    map.insert((7, 8), 326.10494291123285);
    map.insert((7, 9), 683.4085227027382);
    map.insert((7, 10), 1856.1248898219726);
    map.insert((7, 11), 168.27376199721232);
    map.insert((7, 12), 43.69681144563725);
    map.insert((8, 2), 2620.9425537047123);
    map.insert((8, 3), 2614.311160021321);
    map.insert((8, 4), 2082.7399547978107);
    map.insert((8, 5), 1414.1554669901982);
    map.insert((8, 6), 2750.806851355565);
    map.insert((8, 7), 467.53864553468577);
    map.insert((8, 8), 330.1864464111084);
    map.insert((8, 9), 686.8967449070651);
    map.insert((8, 10), 1877.2024353460251);
    map.insert((8, 11), 180.06431599737465);
    map.insert((8, 12), 26.85899097818347);
    map.insert((9, 2), 2618.7979752377323);
    map.insert((9, 3), 2906.8367854467365);
    map.insert((9, 4), 2041.6966040398206);
    map.insert((9, 5), 1432.185588279209);
    map.insert((9, 6), 2736.702492246534);
    map.insert((9, 7), 466.88749960375964);
    map.insert((9, 8), 347.51422643974183);
    map.insert((9, 9), 660.4533208916739);
    map.insert((9, 10), 1897.2549300425326);
    map.insert((9, 11), 184.18489972599193);
    map.insert((9, 12), 25.124716332382043);
    map.insert((10, 2), 2615.7486382583024);
    map.insert((10, 3), 2722.7203649906587);
    map.insert((10, 4), 2111.730300505679);
    map.insert((10, 5), 1466.1291027788347);
    map.insert((10, 6), 2740.8204691195197);
    map.insert((10, 8), 349.37689514515273);
    map.insert((10, 9), 624.7730341762289);
    map.insert((10, 10), 2079.9163166257813);
    map.insert((11, 1), 4954.328464019206);
    map.insert((11, 2), 2759.3437100393116);
    map.insert((11, 3), 2637.7971958871044);
    map.insert((11, 4), 2387.183657008812);
    map.insert((11, 5), 1775.4891425280402);
    map.insert((11, 6), 2807.2301143027685);
    map.insert((12, 1), 4981.158480216904);
    map.insert((12, 2), 2976.210467150659);
    map.insert((12, 3), 2680.872363314877);
    map.insert((12, 4), 1996.9487195698819);
    map.insert((12, 5), 2445.322344311886);
    map.insert((12, 6), 2879.7738015721657);
    map.insert((13, 1), 5027.183147904768);
    map.insert((13, 2), 3314.3720150496724);
    map.insert((13, 3), 2991.169301597712);
    map.insert((13, 4), 2190.8348504874325);
    map.insert((13, 5), 2700.8340113973945);
    map.insert((14, 1), 4864.531944827826);
    map.insert((14, 2), 4167.396280113331);
    map.insert((14, 3), 4346.116562774792);
    map.insert((15, 1), 4859.202027466073);

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
