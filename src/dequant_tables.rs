#[rustfmt::skip]
pub static DAV1D_DQ_TBL: [[[u16; 2]; 256]; 3] = [
    [
        [4, 4], [8, 8], [8, 9], [9, 10],
        [10, 11], [11, 12], [12, 13], [12, 14],
        [13, 15], [14, 16], [15, 17], [16, 18],
        [17, 19], [18, 20], [19, 21], [19, 22],
        [20, 23], [21, 24], [22, 25], [23, 26],
        [24, 27], [25, 28], [26, 29], [26, 30],
        [27, 31], [28, 32], [29, 33], [30, 34],
        [31, 35], [32, 36], [32, 37], [33, 38],
        [34, 39], [35, 40], [36, 41], [37, 42],
        [38, 43], [38, 44], [39, 45], [40, 46],
        [41, 47], [42, 48], [43, 49], [43, 50],
        [44, 51], [45, 52], [46, 53], [47, 54],
        [48, 55], [48, 56], [49, 57], [50, 58],
        [51, 59], [52, 60], [53, 61], [53, 62],
        [54, 63], [55, 64], [56, 65], [57, 66],
        [57, 67], [58, 68], [59, 69], [60, 70],
        [61, 71], [62, 72], [62, 73], [63, 74],
        [64, 75], [65, 76], [66, 77], [66, 78],
        [67, 79], [68, 80], [69, 81], [70, 82],
        [70, 83], [71, 84], [72, 85], [73, 86],
        [74, 87], [74, 88], [75, 89], [76, 90],
        [77, 91], [78, 92], [78, 93], [79, 94],
        [80, 95], [81, 96], [81, 97], [82, 98],
        [83, 99], [84, 100], [85, 101], [85, 102],
        [87, 104], [88, 106], [90, 108], [92, 110],
        [93, 112], [95, 114], [96, 116], [98, 118],
        [99, 120], [101, 122], [102, 124], [104, 126],
        [105, 128], [107, 130], [108, 132], [110, 134],
        [111, 136], [113, 138], [114, 140], [116, 142],
        [117, 144], [118, 146], [120, 148], [121, 150],
        [123, 152], [125, 155], [127, 158], [129, 161],
        [131, 164], [134, 167], [136, 170], [138, 173],
        [140, 176], [142, 179], [144, 182], [146, 185],
        [148, 188], [150, 191], [152, 194], [154, 197],
        [156, 200], [158, 203], [161, 207], [164, 211],
        [166, 215], [169, 219], [172, 223], [174, 227],
        [177, 231], [180, 235], [182, 239], [185, 243],
        [187, 247], [190, 251], [192, 255], [195, 260],
        [199, 265], [202, 270], [205, 275], [208, 280],
        [211, 285], [214, 290], [217, 295], [220, 300],
        [223, 305], [226, 311], [230, 317], [233, 323],
        [237, 329], [240, 335], [243, 341], [247, 347],
        [250, 353], [253, 359], [257, 366], [261, 373],
        [265, 380], [269, 387], [272, 394], [276, 401],
        [280, 408], [284, 416], [288, 424], [292, 432],
        [296, 440], [300, 448], [304, 456], [309, 465],
        [313, 474], [317, 483], [322, 492], [326, 501],
        [330, 510], [335, 520], [340, 530], [344, 540],
        [349, 550], [354, 560], [359, 571], [364, 582],
        [369, 593], [374, 604], [379, 615], [384, 627],
        [389, 639], [395, 651], [400, 663], [406, 676],
        [411, 689], [417, 702], [423, 715], [429, 729],
        [435, 743], [441, 757], [447, 771], [454, 786],
        [461, 801], [467, 816], [475, 832], [482, 848],
        [489, 864], [497, 881], [505, 898], [513, 915],
        [522, 933], [530, 951], [539, 969], [549, 988],
        [559, 1007], [569, 1026], [579, 1046], [590, 1066],
        [602, 1087], [614, 1108], [626, 1129], [640, 1151],
        [654, 1173], [668, 1196], [684, 1219], [700, 1243],
        [717, 1267], [736, 1292], [755, 1317], [775, 1343],
        [796, 1369], [819, 1396], [843, 1423], [869, 1451],
        [896, 1479], [925, 1508], [955, 1537], [988, 1567],
        [1022, 1597], [1058, 1628], [1098, 1660], [1139, 1692],
        [1184, 1725], [1232, 1759], [1282, 1793], [1336, 1828],
    ],
    [
        [4, 4], [9, 9], [10, 11], [13, 13],
        [15, 16], [17, 18], [20, 21], [22, 24],
        [25, 27], [28, 30], [31, 33], [34, 37],
        [37, 40], [40, 44], [43, 48], [47, 51],
        [50, 55], [53, 59], [57, 63], [60, 67],
        [64, 71], [68, 75], [71, 79], [75, 83],
        [78, 88], [82, 92], [86, 96], [90, 100],
        [93, 105], [97, 109], [101, 114], [105, 118],
        [109, 122], [113, 127], [116, 131], [120, 136],
        [124, 140], [128, 145], [132, 149], [136, 154],
        [140, 158], [143, 163], [147, 168], [151, 172],
        [155, 177], [159, 181], [163, 186], [166, 190],
        [170, 195], [174, 199], [178, 204], [182, 208],
        [185, 213], [189, 217], [193, 222], [197, 226],
        [200, 231], [204, 235], [208, 240], [212, 244],
        [215, 249], [219, 253], [223, 258], [226, 262],
        [230, 267], [233, 271], [237, 275], [241, 280],
        [244, 284], [248, 289], [251, 293], [255, 297],
        [259, 302], [262, 306], [266, 311], [269, 315],
        [273, 319], [276, 324], [280, 328], [283, 332],
        [287, 337], [290, 341], [293, 345], [297, 349],
        [300, 354], [304, 358], [307, 362], [310, 367],
        [314, 371], [317, 375], [321, 379], [324, 384],
        [327, 388], [331, 392], [334, 396], [337, 401],
        [343, 409], [350, 417], [356, 425], [362, 433],
        [369, 441], [375, 449], [381, 458], [387, 466],
        [394, 474], [400, 482], [406, 490], [412, 498],
        [418, 506], [424, 514], [430, 523], [436, 531],
        [442, 539], [448, 547], [454, 555], [460, 563],
        [466, 571], [472, 579], [478, 588], [484, 596],
        [490, 604], [499, 616], [507, 628], [516, 640],
        [525, 652], [533, 664], [542, 676], [550, 688],
        [559, 700], [567, 713], [576, 725], [584, 737],
        [592, 749], [601, 761], [609, 773], [617, 785],
        [625, 797], [634, 809], [644, 825], [655, 841],
        [666, 857], [676, 873], [687, 889], [698, 905],
        [708, 922], [718, 938], [729, 954], [739, 970],
        [749, 986], [759, 1002], [770, 1018], [782, 1038],
        [795, 1058], [807, 1078], [819, 1098], [831, 1118],
        [844, 1138], [856, 1158], [868, 1178], [880, 1198],
        [891, 1218], [906, 1242], [920, 1266], [933, 1290],
        [947, 1314], [961, 1338], [975, 1362], [988, 1386],
        [1001, 1411], [1015, 1435], [1030, 1463], [1045, 1491],
        [1061, 1519], [1076, 1547], [1090, 1575], [1105, 1603],
        [1120, 1631], [1137, 1663], [1153, 1695], [1170, 1727],
        [1186, 1759], [1202, 1791], [1218, 1823], [1236, 1859],
        [1253, 1895], [1271, 1931], [1288, 1967], [1306, 2003],
        [1323, 2039], [1342, 2079], [1361, 2119], [1379, 2159],
        [1398, 2199], [1416, 2239], [1436, 2283], [1456, 2327],
        [1476, 2371], [1496, 2415], [1516, 2459], [1537, 2507],
        [1559, 2555], [1580, 2603], [1601, 2651], [1624, 2703],
        [1647, 2755], [1670, 2807], [1692, 2859], [1717, 2915],
        [1741, 2971], [1766, 3027], [1791, 3083], [1817, 3143],
        [1844, 3203], [1871, 3263], [1900, 3327], [1929, 3391],
        [1958, 3455], [1990, 3523], [2021, 3591], [2054, 3659],
        [2088, 3731], [2123, 3803], [2159, 3876], [2197, 3952],
        [2236, 4028], [2276, 4104], [2319, 4184], [2363, 4264],
        [2410, 4348], [2458, 4432], [2508, 4516], [2561, 4604],
        [2616, 4692], [2675, 4784], [2737, 4876], [2802, 4972],
        [2871, 5068], [2944, 5168], [3020, 5268], [3102, 5372],
        [3188, 5476], [3280, 5584], [3375, 5692], [3478, 5804],
        [3586, 5916], [3702, 6032], [3823, 6148], [3953, 6268],
        [4089, 6388], [4236, 6512], [4394, 6640], [4559, 6768],
        [4737, 6900], [4929, 7036], [5130, 7172], [5347, 7312],
    ],
    [
        [4, 4], [12, 13], [18, 19], [25, 27],
        [33, 35], [41, 44], [50, 54], [60, 64],
        [70, 75], [80, 87], [91, 99], [103, 112],
        [115, 126], [127, 139], [140, 154], [153, 168],
        [166, 183], [180, 199], [194, 214], [208, 230],
        [222, 247], [237, 263], [251, 280], [266, 297],
        [281, 314], [296, 331], [312, 349], [327, 366],
        [343, 384], [358, 402], [374, 420], [390, 438],
        [405, 456], [421, 475], [437, 493], [453, 511],
        [469, 530], [484, 548], [500, 567], [516, 586],
        [532, 604], [548, 623], [564, 642], [580, 660],
        [596, 679], [611, 698], [627, 716], [643, 735],
        [659, 753], [674, 772], [690, 791], [706, 809],
        [721, 828], [737, 846], [752, 865], [768, 884],
        [783, 902], [798, 920], [814, 939], [829, 957],
        [844, 976], [859, 994], [874, 1012], [889, 1030],
        [904, 1049], [919, 1067], [934, 1085], [949, 1103],
        [964, 1121], [978, 1139], [993, 1157], [1008, 1175],
        [1022, 1193], [1037, 1211], [1051, 1229], [1065, 1246],
        [1080, 1264], [1094, 1282], [1108, 1299], [1122, 1317],
        [1136, 1335], [1151, 1352], [1165, 1370], [1179, 1387],
        [1192, 1405], [1206, 1422], [1220, 1440], [1234, 1457],
        [1248, 1474], [1261, 1491], [1275, 1509], [1288, 1526],
        [1302, 1543], [1315, 1560], [1329, 1577], [1342, 1595],
        [1368, 1627], [1393, 1660], [1419, 1693], [1444, 1725],
        [1469, 1758], [1494, 1791], [1519, 1824], [1544, 1856],
        [1569, 1889], [1594, 1922], [1618, 1954], [1643, 1987],
        [1668, 2020], [1692, 2052], [1717, 2085], [1741, 2118],
        [1765, 2150], [1789, 2183], [1814, 2216], [1838, 2248],
        [1862, 2281], [1885, 2313], [1909, 2346], [1933, 2378],
        [1957, 2411], [1992, 2459], [2027, 2508], [2061, 2556],
        [2096, 2605], [2130, 2653], [2165, 2701], [2199, 2750],
        [2233, 2798], [2267, 2847], [2300, 2895], [2334, 2943],
        [2367, 2992], [2400, 3040], [2434, 3088], [2467, 3137],
        [2499, 3185], [2532, 3234], [2575, 3298], [2618, 3362],
        [2661, 3426], [2704, 3491], [2746, 3555], [2788, 3619],
        [2830, 3684], [2872, 3748], [2913, 3812], [2954, 3876],
        [2995, 3941], [3036, 4005], [3076, 4069], [3127, 4149],
        [3177, 4230], [3226, 4310], [3275, 4390], [3324, 4470],
        [3373, 4550], [3421, 4631], [3469, 4711], [3517, 4791],
        [3565, 4871], [3621, 4967], [3677, 5064], [3733, 5160],
        [3788, 5256], [3843, 5352], [3897, 5448], [3951, 5544],
        [4005, 5641], [4058, 5737], [4119, 5849], [4181, 5961],
        [4241, 6073], [4301, 6185], [4361, 6297], [4420, 6410],
        [4479, 6522], [4546, 6650], [4612, 6778], [4677, 6906],
        [4742, 7034], [4807, 7162], [4871, 7290], [4942, 7435],
        [5013, 7579], [5083, 7723], [5153, 7867], [5222, 8011],
        [5291, 8155], [5367, 8315], [5442, 8475], [5517, 8635],
        [5591, 8795], [5665, 8956], [5745, 9132], [5825, 9308],
        [5905, 9484], [5984, 9660], [6063, 9836], [6149, 10028],
        [6234, 10220], [6319, 10412], [6404, 10604], [6495, 10812],
        [6587, 11020], [6678, 11228], [6769, 11437], [6867, 11661],
        [6966, 11885], [7064, 12109], [7163, 12333], [7269, 12573],
        [7376, 12813], [7483, 13053], [7599, 13309], [7715, 13565],
        [7832, 13821], [7958, 14093], [8085, 14365], [8214, 14637],
        [8352, 14925], [8492, 15213], [8635, 15502], [8788, 15806],
        [8945, 16110], [9104, 16414], [9275, 16734], [9450, 17054],
        [9639, 17390], [9832, 17726], [10031, 18062], [10245, 18414],
        [10465, 18766], [10702, 19134], [10946, 19502], [11210, 19886],
        [11482, 20270], [11776, 20670], [12081, 21070], [12409, 21486],
        [12750, 21902], [13118, 22334], [13501, 22766], [13913, 23214],
        [14343, 23662], [14807, 24126], [15290, 24590], [15812, 25070],
        [16356, 25551], [16943, 26047], [17575, 26559], [18237, 27071],
        [18949, 27599], [19718, 28143], [20521, 28687], [21387, 29247],
    ],
];
