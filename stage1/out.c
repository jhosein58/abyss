#include <stdio.h>
#include <stdint.h>
#include <stdbool.h>
#include <string.h>
#include <stdlib.h>

void print(uint8_t *s) {
    printf("%s", (const char *)s);
}

void print_char(uint8_t c) {
    putchar((int)c);
}

void print_i8(int8_t v) {
    printf("%d", (int)v);
}

void print_i16(int16_t v) {
    printf("%d", (int)v);
}

void print_i32(int32_t v) {
    printf("%d", v);
}

void print_i64(int64_t v) {
    printf("%lld", (long long)v);
}

void print_u8(uint8_t v) {
    printf("%u", (unsigned int)v);
}

void print_u16(uint16_t v) {
    printf("%u", (unsigned int)v);
}

void print_u32(uint32_t v) {
    printf("%u", v);
}

void print_u64(uint64_t v) {
    printf("%llu", (unsigned long long)v);
}

void print_f16(_Float16 v) {
    printf("%f", (double)v);
}

void print_f32(float v) {
    printf("%f", (double)v);
}

void print_f64(double v) {
    printf("%f", v);
}


void print_bool(bool b) {
    printf("%s", b ? "true" : "false");
}

// ------> String

uint64_t str_len(uint8_t *s) {
    if (!s) return 0;
    return (uint64_t)strlen((char *)s);
}

bool str_eq(uint8_t *a, uint8_t *b) {
    if (a == b) return true;
    if (!a || !b) return false;
    return strcmp((const char *)a, (const char *)b) == 0;
}

// ------> mem
void mem_copy(uint8_t *dest, uint8_t *src, uint64_t n) {
    if (!dest || !src || n == 0) return;
    if (n > 100 * 1024 * 1024) return; 
    memcpy(dest, src, (size_t)n);
}

void mem_set(uint8_t *dest, uint8_t val, uint64_t n) {
    memset(dest, (int)val, (size_t)n);
}

// ------> File Stream

uint8_t *file_open(uint8_t *path, uint8_t *mode) {
    return (uint8_t*)(fopen(path, mode));
}

int32_t file_seek(uint8_t *handle, int64_t offset, int32_t w) {
    return fseek((FILE*)handle, offset, w);
}

int64_t file_tell(uint8_t *handle) {
    return ftell((FILE*)handle);
}

uint64_t file_read(uint8_t *handle, uint8_t *ptr, uint64_t size, uint64_t count) {
    return fread(ptr, size ,count, (FILE*)handle);
}

int32_t file_close(uint8_t *handle) {
    return fclose((FILE*)handle);
}

uint64_t file_write(uint8_t *handle, uint8_t *ptr, uint64_t size, uint64_t count) {
    return fwrite(ptr, size, count, (FILE*)handle);
}

// ------> Process

int32_t compile_and_run(uint8_t *path) {
    if (!path) return -1;

    const char *p = (const char *)path;
    char out_path[1024];

    const char *last_slash = strrchr(p, '/');
    if (last_slash) {
        size_t dir_len = (size_t)(last_slash - p + 1);
        if (dir_len + sizeof("stg1_out") >= sizeof(out_path)) return -1;

        memcpy(out_path, p, dir_len);
        strcpy(out_path + dir_len, "stg1_out");
    } else {
        snprintf(out_path, sizeof(out_path), "./stg1_out");
    }

    char compile_cmd[2048];
    snprintf(compile_cmd, sizeof(compile_cmd), "gcc \"%s\" -o \"%s\" 2>&1", p, out_path);

    fprintf(stderr, "[CMD] %s\n", compile_cmd);
    fflush(stderr);

    FILE *fp = popen(compile_cmd, "r");
    if (!fp) {
        fprintf(stderr, "Failed to run popen for gcc\n");
        fflush(stderr);
        return -1;
    }

    char buffer[256];
    int has_compiler_output = 0;
    while (fgets(buffer, sizeof(buffer), fp) != NULL) {
        if (!has_compiler_output) {
            fprintf(stderr, "\n--- GCC Output ---\n");
            has_compiler_output = 1;
        }
        fputs(buffer, stderr);
        fflush(stderr); 
    }

    int compile_status = pclose(fp);
    if (compile_status != 0) {
        fprintf(stderr, "\n[GCC Exit Code]: %d\n", compile_status);
        fflush(stderr);
        return compile_status;
    }

    char run_cmd[1050];
    snprintf(run_cmd, sizeof(run_cmd), "\"%s\"", out_path);
    return (int32_t)system(run_cmd);
}// Struct Declarations
typedef struct S_12 S_12;
typedef struct S_14 S_14;
typedef struct S_23 S_23;
typedef struct S_42 S_42;
typedef struct S_45 S_45;
typedef struct S_47 S_47;
typedef struct S_49 S_49;
typedef struct S_50 S_50;
typedef struct S_51 S_51;
typedef struct S_52 S_52;
typedef struct S_53 S_53;
typedef struct S_112 S_112;
typedef struct S_54 S_54;
typedef struct S_55 S_55;
typedef struct S_56 S_56;
typedef struct S_57 S_57;
typedef struct S_58 S_58;
typedef struct S_59 S_59;
typedef struct S_197 S_197;
typedef struct S_222 S_222;
typedef struct S_332 S_332;
typedef struct S_333 S_333;
typedef struct S_351 S_351;
typedef struct S_430 S_430;
typedef struct S_449 S_449;
typedef struct S_483 S_483;
typedef struct S_567 S_567;
typedef struct S_568 S_568;

// Struct Definitions
struct S_12 {
    uint8_t* _f551;
    uint8_t* _f552;
    uint64_t _f553;
    uint64_t _f554;
};

struct S_14 {
    S_12* _f104;
    uint64_t _f548;
};

struct S_23 {
    uint8_t* _f566;
    uint64_t _f567;
};

struct S_42 {
    uint64_t _f554;
    uint8_t* _f566;
    uint64_t _f567;
};

struct S_45 {
    uint64_t _f554;
    uint32_t* _f566;
    uint64_t _f567;
};

struct S_47 {
    uint64_t _f554;
    S_23* _f566;
    uint64_t _f567;
};

struct S_49 {
    uint64_t _f554;
    bool* _f566;
    uint64_t _f567;
};

struct S_50 {
    S_45 _f567;
    S_42 _f600;
    S_45 _f601;
    S_47 _f602;
    S_49 _f603;
};

struct S_51 {
    uint32_t _f605;
    S_42 _f606;
    S_45 _f607;
    S_45 _f608;
    S_45 _f609;
};

struct S_52 {
    uint64_t _f554;
    uint64_t _f578;
    S_47 _f610;
    uint32_t* _f611;
};

struct S_53 {
    S_42 _f606;
    S_45 _f609;
    S_45 _f615;
};

struct S_112 {
    uint8_t _f547;
    uint8_t _f566;
    uint8_t _f604;
    uint8_t _f636;
    uint8_t _f637;
    uint8_t _f638;
    uint8_t _f639;
    uint8_t _f640;
    uint8_t _f641;
    uint8_t _f645;
    uint8_t _f646;
    uint8_t _f647;
    uint8_t _f648;
    uint8_t _f649;
    uint8_t _f650;
    uint8_t _f651;
};

struct S_54 {
    S_45 _f611;
    S_53 _f612;
    uint32_t _f613;
    uint32_t _f614;
};

struct S_55 {
    S_45 _f587;
    S_45 _f616;
    S_42 _f617;
    S_45 _f618;
    S_45 _f619;
    S_45 _f620;
};

struct S_56 {
    S_45 _f621;
};

struct S_57 {
    S_50 _f582;
    S_51 _f583;
    S_45 _f584;
    S_45 _f585;
    S_52 _f586;
    S_54 _f587;
    S_55 _f588;
    S_56 _f589;
    S_56 _f590;
    S_56 _f591;
    S_56 _f592;
    S_56 _f593;
    S_56 _f594;
    S_56 _f595;
    S_56 _f596;
    S_56 _f597;
    S_56 _f598;
    S_56 _f599;
};

struct S_58 {
    S_42 _f622;
    S_42 _f623;
    S_42 _f624;
    S_42 _f625;
    S_45 _f626;
    uint32_t _f627;
    bool _f628;
    uint32_t _f629;
};

struct S_59 {
    S_58 _f542;
    S_57 _f579;
    uint32_t _f580;
};

struct S_197 {
    S_23 _f536;
    uint64_t _f664;
    bool _f666;
};

struct S_222 {
    uint8_t _f636;
    uint8_t _f667;
    uint8_t _f670;
    uint8_t _f672;
    uint8_t _f673;
    uint8_t _f674;
    uint8_t _f675;
    uint8_t _f676;
    uint8_t _f677;
    uint8_t _f678;
    uint8_t _f679;
    uint8_t _f680;
    uint8_t _f681;
    uint8_t _f682;
    uint8_t _f683;
    uint8_t _f684;
    uint8_t _f685;
    uint8_t _f686;
    uint8_t _f687;
    uint8_t _f688;
    uint8_t _f689;
    uint8_t _f690;
    uint8_t _f691;
    uint8_t _f692;
    uint8_t _f693;
    uint8_t _f694;
    uint8_t _f695;
    uint8_t _f696;
    uint8_t _f697;
    uint8_t _f698;
    uint8_t _f699;
    uint8_t _f700;
    uint8_t _f701;
    uint8_t _f702;
    uint8_t _f703;
    uint8_t _f704;
    uint8_t _f705;
    uint8_t _f706;
    uint8_t _f707;
    uint8_t _f708;
    uint8_t _f709;
    uint8_t _f710;
    uint8_t _f711;
    uint8_t _f712;
    uint8_t _f713;
    uint8_t _f714;
    uint8_t _f715;
    uint8_t _f716;
    uint8_t _f717;
    uint8_t _f718;
    uint8_t _f719;
    uint8_t _f720;
    uint8_t _f721;
    uint8_t _f722;
    uint8_t _f723;
    uint8_t _f724;
    uint8_t _f725;
    uint8_t _f726;
    uint8_t _f727;
    uint8_t _f728;
    uint8_t _f729;
    uint8_t _f730;
};

struct S_332 {
    S_45 _f843;
    S_45 _f844;
};

struct S_333 {
    S_50* _f582;
    S_51* _f583;
    uint32_t _f658;
    uint32_t _f664;
    S_57* _f748;
    S_332 _f838;
    uint32_t _f840;
    uint32_t _f841;
    bool _f842;
};

struct S_351 {
    uint8_t _f641;
    uint8_t _f650;
    uint8_t _f651;
    uint8_t _f670;
    uint8_t _f672;
    uint8_t _f673;
    uint8_t _f674;
    uint8_t _f675;
    uint8_t _f773;
    uint8_t _f850;
    uint8_t _f851;
    uint8_t _f852;
    uint8_t _f853;
    uint8_t _f854;
    uint8_t _f855;
    uint8_t _f856;
    uint8_t _f857;
    uint8_t _f858;
    uint8_t _f859;
    uint8_t _f860;
    uint8_t _f861;
    uint8_t _f862;
    uint8_t _f863;
    uint8_t _f864;
    uint8_t _f865;
    uint8_t _f866;
    uint8_t _f867;
    uint8_t _f868;
    uint8_t _f869;
    uint8_t _f870;
    uint8_t _f871;
    uint8_t _f872;
    uint8_t _f873;
    uint8_t _f874;
    uint8_t _f875;
    uint8_t _f876;
    uint8_t _f877;
    uint8_t _f878;
    uint8_t _f879;
    uint8_t _f880;
    uint8_t _f881;
    uint8_t _f882;
    uint8_t _f883;
    uint8_t _f884;
    uint8_t _f885;
    uint8_t _f886;
    uint8_t _f887;
    uint8_t _f888;
    uint8_t _f889;
    uint8_t _f890;
    uint8_t _f891;
    uint8_t _f892;
    uint8_t _f893;
    uint8_t _f894;
    uint8_t _f895;
    uint8_t _f896;
    uint8_t _f897;
    uint8_t _f898;
    uint8_t _f899;
    uint8_t _f900;
    uint8_t _f901;
    uint8_t _f902;
    uint8_t _f903;
    uint8_t _f904;
};

struct S_430 {
    uint32_t* _f566;
    uint64_t _f567;
};

struct S_449 {
    uint8_t _f889;
    uint8_t _f898;
    uint8_t _f902;
    uint8_t _f939;
    uint8_t _f947;
    uint8_t _f948;
    uint8_t _f949;
    uint8_t _f950;
    uint8_t _f951;
    uint8_t _f952;
    uint8_t _f953;
    uint8_t _f954;
    uint8_t _f955;
    uint8_t _f956;
    uint8_t _f957;
    uint8_t _f958;
    uint8_t _f959;
};

struct S_483 {
    bool _f847;
    uint8_t _f848;
    uint8_t _f849;
};

struct S_567 {
    S_14* _f533;
    uint64_t _f554;
    uint8_t* _f566;
    uint64_t _f567;
};

struct S_568 {
    S_567 _f564;
};


// Forward Declarations
void sym_6(void);
S_14 sym_69(uint64_t sym_1153);
S_12* sym_68(uint64_t sym_1155, uint8_t* sym_1156);
uint64_t sym_64(void);
S_23 sym_82(S_14* sym_1160, uint8_t* sym_1161);
S_23 sym_49(void);
int32_t sym_81(void);
int32_t sym_80(void);
uint8_t* sym_70(S_14* sym_1173, uint64_t sym_1174);
uint64_t sym_67(uint64_t sym_1180);
S_59 sym_311(void);
S_57 sym_215(void);
S_50 sym_11(void);
S_42 sym_348(void);
S_42 sym_367(void);
S_42 sym_368(uint64_t sym_1186);
uint64_t sym_347(void);
uint64_t sym_577(void);
S_45 sym_390(void);
S_45 sym_409(void);
S_45 sym_410(uint64_t sym_1190);
uint64_t sym_389(void);
uint64_t sym_523(void);
S_47 sym_432(void);
S_47 sym_451(void);
S_47 sym_452(uint64_t sym_1193);
uint64_t sym_431(void);
uint64_t sym_47(void);
S_49 sym_474(void);
S_49 sym_493(void);
S_49 sym_494(uint64_t sym_1196);
uint64_t sym_473(void);
uint64_t sym_589(void);
S_51 sym_90(void);
uint32_t sym_85(void);
uint32_t sym_524(void);
S_52 sym_207(void);
S_52 sym_208(uint64_t sym_1199);
S_54 sym_257(void);
S_53 sym_247(void);
S_112 sym_234(void);
uint32_t sym_250(S_53* sym_1211, uint8_t sym_1212, uint32_t sym_1213);
void sym_352(S_42* sym_1215, uint8_t sym_1216);
void sym_372(S_42* sym_1217, uint8_t sym_1218);
void sym_371(S_42* sym_1219);
void sym_370(S_42* sym_1220, uint64_t sym_1221);
void sym_394(S_45* sym_1227, uint32_t sym_1228);
void sym_414(S_45* sym_1229, uint32_t sym_1230);
void sym_413(S_45* sym_1231);
void sym_412(S_45* sym_1232, uint64_t sym_1233);
S_45 sym_391(uint64_t sym_1237);
uint32_t sym_236(void);
S_55 sym_296(void);
S_56 sym_534(void);
S_58 sym_327(void);
uint32_t sym_313(S_59* sym_1238, S_14* sym_1239, S_23 sym_1240, S_23 sym_1241);
uint64_t sym_14(S_50* sym_1245);
void sym_43(S_50* sym_1246, S_23 sym_1247);
S_197 sym_21(S_23 sym_1255);
bool sym_22(S_197* sym_1256);
uint8_t sym_24(S_197* sym_1257);
uint8_t sym_23(S_197* sym_1258, uint64_t sym_1259);
bool sym_32(uint8_t sym_1260);
void sym_34(S_197* sym_1261);
void sym_26(S_197* sym_1262);
bool sym_33(uint8_t sym_1263);
void sym_35(S_197* sym_1264);
uint8_t sym_25(S_197* sym_1265);
void sym_36(S_197* sym_1266);
bool sym_27(uint8_t sym_1267);
void sym_37(S_197* sym_1268, S_50* sym_1269);
bool sym_28(uint8_t sym_1278);
S_23 sym_48(uint8_t* sym_1279, uint64_t sym_1280);
void sym_13(S_50* sym_1281, uint8_t sym_1282, uint32_t sym_1283, uint32_t sym_1284, S_23 sym_1285, bool sym_1286);
void sym_436(S_47* sym_1287, S_23 sym_1288);
void sym_456(S_47* sym_1289, S_23 sym_1290);
void sym_455(S_47* sym_1291);
void sym_454(S_47* sym_1292, uint64_t sym_1293);
void sym_478(S_49* sym_1297, bool sym_1298);
void sym_498(S_49* sym_1299, bool sym_1300);
void sym_497(S_49* sym_1301);
void sym_496(S_49* sym_1302, uint64_t sym_1303);
S_222 sym_8(void);
void sym_38(S_197* sym_1307, S_50* sym_1308);
void sym_39(S_197* sym_1312, S_50* sym_1313);
bool sym_30(uint8_t sym_1317);
bool sym_29(uint8_t sym_1318);
void sym_41(S_197* sym_1319, S_50* sym_1320);
bool sym_31(uint8_t sym_1325);
uint8_t sym_40(S_23 sym_1326);
bool sym_54(S_23 sym_1328, S_23 sym_1329);
S_23 sym_50(uint8_t* sym_1331);
uint8_t sym_42(S_197* sym_1333);
void sym_567(S_57* sym_1336, S_14* sym_1337, uint32_t sym_1338, S_23 sym_1339, uint32_t sym_1340, uint32_t sym_1341);
uint32_t sym_211(S_52* sym_1361, S_23 sym_1362);
void sym_210(S_52* sym_1370);
uint32_t sym_205(void);
S_23 sym_438(S_47* sym_1382, uint64_t sym_1383);
S_23 sym_458(S_47* sym_1384, uint64_t sym_1385);
uint32_t sym_561(S_23 sym_1386);
uint32_t sym_396(S_45* sym_1391, uint64_t sym_1392);
uint32_t sym_416(S_45* sym_1393, uint64_t sym_1394);
S_23 sym_212(S_52* sym_1395, uint32_t sym_1396);
void sym_566(S_57* sym_1397, S_14* sym_1398, uint32_t sym_1399, S_23 sym_1400, uint32_t sym_1401, uint32_t sym_1402, S_45* sym_1403, S_45* sym_1404, S_45* sym_1405, S_45* sym_1406, S_45* sym_1407);
uint8_t sym_15(S_50* sym_1435, uint32_t sym_1436);
uint8_t sym_354(S_42* sym_1437, uint64_t sym_1438);
uint8_t sym_374(S_42* sym_1439, uint64_t sym_1440);
S_23 sym_18(S_50* sym_1441, uint32_t sym_1442);
bool sym_562(S_23 sym_1443, S_23 sym_1444);
uint32_t sym_218(S_57* sym_1446, uint32_t sym_1447, uint32_t sym_1448, uint32_t sym_1449, uint32_t sym_1450);
uint32_t sym_537(S_56* sym_1452, uint32_t sym_1453);
uint32_t sym_532(void);
S_23 sym_563(S_23 sym_1455);
uint32_t sym_565(S_57* sym_1458, S_14* sym_1459, S_23 sym_1460, S_23 sym_1461, S_45* sym_1462, S_45* sym_1463, S_45* sym_1464, S_45* sym_1465);
S_23 sym_564(S_14* sym_1473, S_23 sym_1474, S_23 sym_1475);
void sym_392(S_45* sym_1504);
void sym_411(S_45* sym_1505);
uint32_t sym_221(S_57* sym_1506, uint32_t sym_1507, uint32_t sym_1508);
uint32_t sym_536(S_56* sym_1515);
uint32_t sym_538(S_56* sym_1516, uint32_t sym_1517);
uint32_t sym_220(S_57* sym_1518, uint32_t sym_1519, uint32_t sym_1520, uint32_t sym_1521);
void sym_397(S_45* sym_1523, uint64_t sym_1524, uint32_t sym_1525);
void sym_417(S_45* sym_1526, uint64_t sym_1527, uint32_t sym_1528);
uint32_t sym_314(S_59* sym_1529, uint32_t sym_1530, S_23 sym_1531);
void sym_315(S_59* sym_1533, uint32_t sym_1534);
uint32_t sym_92(S_51* sym_1542);
S_333 sym_103(S_57* sym_1543, S_50* sym_1544, S_51* sym_1545, uint32_t sym_1546, uint32_t sym_1547, uint32_t sym_1548, uint32_t sym_1549);
S_332 sym_526(void);
uint32_t sym_117(S_333* sym_1550, uint8_t sym_1551);
uint32_t sym_118(S_333* sym_1555);
uint8_t sym_106(S_333* sym_1558);
uint32_t sym_125(S_333* sym_1559);
S_351 sym_87(void);
uint32_t sym_114(S_23 sym_1562);
S_23 sym_112(S_333* sym_1566);
uint32_t sym_108(S_333* sym_1567);
uint32_t sym_93(S_51* sym_1569, uint8_t sym_1570, uint32_t sym_1571, uint32_t sym_1572, uint32_t sym_1573);
uint32_t sym_136(S_333* sym_1575);
uint32_t sym_137(S_333* sym_1579);
uint32_t sym_138(S_333* sym_1583);
uint32_t sym_139(S_333* sym_1585);
uint32_t sym_126(S_333* sym_1587);
uint32_t sym_104(S_333* sym_1594, uint32_t sym_1595);
uint32_t sym_531(S_332* sym_1597, uint32_t sym_1598);
uint64_t sym_405(S_45* sym_1602);
uint64_t sym_425(S_45* sym_1603);
void sym_224(S_57* sym_1604, uint32_t sym_1605, uint32_t sym_1606);
void sym_541(S_56* sym_1607, uint32_t sym_1608, uint32_t sym_1609, uint32_t sym_1610);
void sym_540(S_56* sym_1612, uint32_t sym_1613, uint32_t sym_1614);
uint32_t sym_127(S_333* sym_1616);
bool sym_109(S_333* sym_1636, uint8_t sym_1637);
uint32_t sym_528(S_332* sym_1638);
uint32_t sym_124(S_333* sym_1639, uint32_t sym_1640);
uint32_t sym_128(S_333* sym_1645);
uint64_t sym_401(S_45* sym_1655);
uint64_t sym_421(S_45* sym_1656);
uint32_t* sym_404(S_45* sym_1657, uint64_t sym_1658);
uint32_t* sym_424(S_45* sym_1659, uint64_t sym_1660);
uint32_t sym_216(S_57* sym_1661, S_430 sym_1662);
void sym_400(S_45* sym_1665, uint32_t* sym_1666, uint64_t sym_1667);
void sym_420(S_45* sym_1668, uint32_t* sym_1669, uint64_t sym_1670);
void sym_402(S_45* sym_1675, uint64_t sym_1676);
void sym_422(S_45* sym_1677, uint64_t sym_1678);
void sym_530(S_332* sym_1679, uint32_t sym_1680);
bool sym_123(S_333* sym_1681);
bool sym_19(S_50* sym_1688, uint32_t sym_1689);
bool sym_480(S_49* sym_1690, uint64_t sym_1691);
bool sym_500(S_49* sym_1692, uint64_t sym_1693);
bool sym_105(S_333* sym_1694);
void sym_122(S_333* sym_1695, uint32_t sym_1696);
uint8_t sym_94(S_51* sym_1700, uint32_t sym_1701);
uint32_t sym_95(S_51* sym_1702, uint32_t sym_1703);
uint32_t sym_219(S_57* sym_1704, uint32_t sym_1705, uint32_t sym_1706, uint32_t sym_1707);
void sym_529(S_332* sym_1709, uint32_t sym_1710, uint32_t sym_1711);
bool sym_110(S_333* sym_1712, uint8_t sym_1713);
uint32_t sym_120(S_333* sym_1714);
uint32_t sym_121(S_333* sym_1721);
uint32_t sym_140(S_333* sym_1726);
uint32_t sym_141(S_333* sym_1728);
uint32_t sym_130(S_333* sym_1730);
bool sym_111(S_333* sym_1734);
uint32_t sym_142(S_333* sym_1735);
S_449 sym_543(void);
uint8_t sym_107(S_333* sym_1750, uint32_t sym_1751);
uint32_t sym_143(S_333* sym_1753);
uint32_t sym_129(S_333* sym_1768, uint8_t sym_1769);
bool sym_549(uint8_t sym_1774);
S_483 sym_548(uint8_t sym_1776);
S_483 sym_546(uint8_t sym_1779);
S_483 sym_545(uint8_t sym_1781);
S_483 sym_547(void);
uint32_t sym_119(S_333* sym_1782, uint8_t sym_1783, uint32_t sym_1784, uint8_t sym_1785);
uint32_t sym_132(S_333* sym_1787, uint32_t sym_1788, uint8_t sym_1789);
uint32_t sym_223(S_57* sym_1796, uint32_t sym_1797);
void sym_222(S_57* sym_1798, uint32_t sym_1799, uint32_t sym_1800);
void sym_539(S_56* sym_1801, uint32_t sym_1802, uint32_t sym_1803);
uint32_t sym_133(S_333* sym_1804, uint32_t sym_1805, uint8_t sym_1806);
uint32_t sym_134(S_333* sym_1819, uint32_t sym_1820, uint8_t sym_1821);
uint32_t sym_135(S_333* sym_1824, uint32_t sym_1825, uint8_t sym_1826);
uint32_t sym_144(S_333* sym_1833, uint32_t sym_1834, uint8_t sym_1835);
uint32_t sym_131(S_333* sym_1838, uint8_t sym_1839, uint32_t sym_1840, uint8_t sym_1841);
void sym_527(S_332* sym_1846);
void sym_316(S_59* sym_1847, uint32_t sym_1848);
void sym_159(S_59* sym_1851, uint32_t sym_1852, uint32_t sym_1853);
uint32_t sym_300(S_55* sym_1858);
uint32_t sym_294(void);
void sym_160(S_59* sym_1860, uint32_t sym_1861, uint32_t sym_1862);
void sym_161(S_57* sym_1867, S_51* sym_1868, uint32_t sym_1869);
uint32_t sym_299(S_55* sym_1873, uint32_t sym_1874);
void sym_298(S_55* sym_1876, uint32_t sym_1877, uint32_t sym_1878);
uint32_t sym_304(S_55* sym_1880, S_54* sym_1881, uint32_t sym_1882, uint32_t sym_1883);
uint32_t sym_302(S_55* sym_1887, uint32_t sym_1888);
uint32_t sym_305(S_55* sym_1892, S_54* sym_1893, uint32_t sym_1894, uint32_t sym_1895);
uint8_t sym_260(S_54* sym_1928, uint32_t sym_1929);
uint32_t sym_282(S_54* sym_1930, uint32_t sym_1931);
uint32_t sym_261(S_54* sym_1932, uint32_t sym_1933);
uint32_t sym_303(S_55* sym_1934, S_54* sym_1935, uint32_t sym_1936, uint32_t sym_1937);
uint32_t sym_243(void);
void sym_355(S_42* sym_1949, uint64_t sym_1950, uint8_t sym_1951);
void sym_375(S_42* sym_1952, uint64_t sym_1953, uint8_t sym_1954);
uint32_t sym_273(S_54* sym_1955, uint32_t sym_1956);
uint32_t sym_272(S_54* sym_1957, uint8_t sym_1958, uint32_t sym_1959);
void sym_263(S_54* sym_1965);
uint32_t sym_249(S_53* sym_1975);
uint32_t sym_245(void);
uint32_t sym_262(S_54* sym_1976, uint32_t sym_1977);
uint32_t sym_251(uint32_t sym_1999, uint32_t sym_2000);
bool sym_252(S_53* sym_2001, uint32_t sym_2002, uint8_t sym_2003, uint32_t sym_2004);
uint32_t sym_244(void);
uint32_t sym_277(S_54* sym_2005, uint32_t sym_2006);
uint32_t sym_284(S_54* sym_2007, uint32_t sym_2008);
uint32_t sym_283(S_54* sym_2010, uint32_t sym_2011);
uint32_t sym_278(S_54* sym_2013, uint32_t sym_2014, uint32_t sym_2015);
bool sym_253(S_53* sym_2023, uint32_t sym_2024, uint8_t sym_2025, uint32_t sym_2026, uint32_t sym_2027);
S_430 sym_290(S_54* sym_2031, uint32_t sym_2032);
uint32_t sym_281(S_54* sym_2036, S_430 sym_2037);
void sym_280(S_430 sym_2048);
bool sym_255(S_53* sym_2058, uint32_t sym_2059, uint8_t sym_2060, S_430 sym_2061, uint32_t sym_2062);
uint32_t sym_238(void);
void sym_162(S_59* sym_2068, uint32_t sym_2069);
uint32_t sym_301(S_55* sym_2083, uint32_t sym_2084);
uint32_t sym_158(S_54* sym_2086, S_23 sym_2087);
bool sym_157(S_23 sym_2092, S_23 sym_2093);
uint32_t sym_267(void);
uint32_t sym_240(void);
uint32_t sym_269(void);
uint32_t sym_242(void);
uint32_t sym_274(S_54* sym_2095, uint16_t sym_2096);
uint32_t sym_275(S_54* sym_2097, uint16_t sym_2098);
uint32_t sym_276(S_54* sym_2099, uint16_t sym_2100);
uint32_t sym_241(void);
void sym_226(S_57* sym_2101, uint32_t sym_2102, uint32_t sym_2103);
uint32_t sym_225(S_57* sym_2104, uint32_t sym_2105);
uint32_t sym_318(S_59* sym_2106, uint32_t sym_2107);
void sym_317(S_59* sym_2109, uint32_t sym_2110);
uint32_t sym_306(S_55* sym_2117, uint32_t sym_2118);
bool sym_229(S_57* sym_2120, uint32_t sym_2121);
void sym_228(S_57* sym_2122, uint32_t sym_2123, bool sym_2124);
uint32_t sym_227(S_57* sym_2126, uint32_t sym_2127);
void sym_163(S_57* sym_2128, S_51* sym_2129, uint32_t sym_2130);
uint32_t sym_96(S_51* sym_2143, uint32_t sym_2144);
uint32_t sym_97(S_51* sym_2145, uint32_t sym_2146);
void sym_164(S_57* sym_2147, S_51* sym_2148, uint32_t sym_2149);
void sym_165(S_57* sym_2156, S_51* sym_2157, uint32_t sym_2158, uint32_t sym_2159);
S_430 sym_217(S_57* sym_2180, uint32_t sym_2181);
uint32_t sym_237(void);
uint32_t sym_279(S_54* sym_2184, S_430 sym_2185, uint32_t sym_2186, bool sym_2187);
bool sym_254(S_53* sym_2200, uint32_t sym_2201, uint8_t sym_2202, S_430 sym_2203, uint32_t sym_2204, uint32_t sym_2205);
void sym_166(S_57* sym_2211, S_51* sym_2212, uint32_t sym_2213, uint32_t sym_2214);
void sym_167(S_57* sym_2221, S_51* sym_2222, uint32_t sym_2223);
uint32_t sym_307(S_55* sym_2239, S_54* sym_2240, uint32_t sym_2241);
uint32_t sym_285(S_54* sym_2265, uint32_t sym_2266);
S_430 sym_288(S_54* sym_2268, uint32_t sym_2269);
uint32_t sym_286(S_54* sym_2273, uint32_t sym_2274);
void sym_168(S_57* sym_2277, S_51* sym_2278, uint32_t sym_2279);
void sym_169(S_57* sym_2287, S_51* sym_2288, uint32_t sym_2289);
void sym_170(S_57* sym_2301, S_51* sym_2302, uint32_t sym_2303);
void sym_171(S_57* sym_2311, S_51* sym_2312, uint32_t sym_2313);
uint32_t sym_239(void);
void sym_172(S_57* sym_2315, S_51* sym_2316, uint32_t sym_2317);
void sym_173(S_57* sym_2319, S_51* sym_2320, uint32_t sym_2321);
void sym_174(S_57* sym_2325, S_51* sym_2326, uint32_t sym_2327);
void sym_175(S_57* sym_2331, S_51* sym_2332, uint32_t sym_2333);
void sym_176(S_57* sym_2337, S_51* sym_2338, uint32_t sym_2339);
void sym_177(S_57* sym_2347, S_51* sym_2348, uint32_t sym_2349);
void sym_178(S_57* sym_2355, S_51* sym_2356, uint32_t sym_2357);
void sym_179(S_57* sym_2359, S_51* sym_2360, uint32_t sym_2361);
void sym_180(S_57* sym_2369, S_51* sym_2370, uint32_t sym_2371);
void sym_183(S_57* sym_2375, S_51* sym_2376, uint32_t sym_2377);
void sym_184(S_57* sym_2381, S_51* sym_2382, uint32_t sym_2383);
void sym_185(S_57* sym_2401, S_51* sym_2402, uint32_t sym_2403);
void sym_186(S_57* sym_2421, S_51* sym_2422, uint32_t sym_2423);
void sym_182(S_57* sym_2442, S_51* sym_2443, uint32_t sym_2444);
void sym_181(S_57* sym_2450, S_51* sym_2451, uint32_t sym_2452);
void sym_320(S_59* sym_2458, uint32_t sym_2459);
void sym_330(S_58* sym_2460, uint32_t sym_2461);
void sym_321(S_59* sym_2462, S_14* sym_2463, uint32_t sym_2464);
void sym_576(S_57* sym_2466, S_58* sym_2467, S_14* sym_2468, uint32_t sym_2469, S_45* sym_2470);
bool sym_572(S_45* sym_2511, uint32_t sym_2512);
uint32_t sym_571(S_57* sym_2514, uint32_t sym_2515);
S_23 sym_570(S_54* sym_2517, S_14* sym_2518, uint32_t sym_2519);
S_568 sym_56(S_14* sym_2526);
S_567 sym_515(S_14* sym_2527);
void sym_57(S_568* sym_2531, S_23 sym_2532);
void sym_518(S_567* sym_2534, uint8_t sym_2535);
void sym_517(S_567* sym_2536);
uint8_t* sym_72(S_14* sym_2541, uint8_t* sym_2542, uint64_t sym_2543, uint64_t sym_2544);
S_23 sym_58(S_568* sym_2554);
S_23 sym_293(S_54* sym_2555, S_14* sym_2556, uint32_t sym_2557);
void sym_60(S_568* sym_2559, uint32_t sym_2560);
void sym_59(S_568* sym_2564, uint8_t sym_2565);
bool sym_287(S_54* sym_2566, uint32_t sym_2567);
void sym_332(S_58* sym_2570, S_23 sym_2571, S_23 sym_2572, S_23 sym_2573);
void sym_331(S_58* sym_2574, S_23 sym_2575, S_23 sym_2576, S_23 sym_2577);
void sym_322(S_42* sym_2578, S_23 sym_2579);
void sym_323(S_42* sym_2581, uint8_t sym_2582);
S_23 sym_575(S_57* sym_2583, S_58* sym_2584, S_14* sym_2585, uint32_t sym_2586, S_45* sym_2587, S_45* sym_2588, S_45* sym_2589);
void sym_334(S_58* sym_2682, S_23 sym_2683, S_23 sym_2684, S_23 sym_2685, bool sym_2686);
void sym_329(S_58* sym_2687);
void sym_574(S_57* sym_2689, uint32_t sym_2690, S_45* sym_2691);
S_23 sym_573(S_351 sym_2699, uint8_t sym_2700);
void sym_335(S_58* sym_2701, S_23 sym_2702);
void sym_338(S_58* sym_2703, S_23 sym_2704);
void sym_339(S_58* sym_2705);
void sym_341(S_58* sym_2706);
void sym_340(S_58* sym_2707, S_23 sym_2708);
void sym_336(S_58* sym_2709, S_23 sym_2710, bool sym_2711);
void sym_333(S_58* sym_2712);
S_45 sym_596(S_54* sym_2713, S_45* sym_2714);
uint32_t sym_259(S_54* sym_2721);
void sym_595(S_54* sym_2722, S_45* sym_2723, S_42* sym_2724, uint32_t sym_2725);
void sym_350(S_42* sym_2736);
void sym_369(S_42* sym_2737);
void sym_343(S_58* sym_2738, uint32_t sym_2739, S_23 sym_2740, S_23 sym_2741);
void sym_342(S_58* sym_2743, S_23 sym_2744);
S_23 sym_337(S_58* sym_2745, S_23 sym_2746);
S_23 sym_325(S_42* sym_2748);
void sym_324(S_42* sym_2749, uint32_t sym_2750);
bool sym_83(uint8_t* sym_2755, S_23 sym_2756);

// Implementations
void sym_6(void) {
    print(((uint8_t*)"--> dummy main \n"));
    S_14 sym_1145 = sym_69(((1024 * 1024) * 16));
    uint8_t* sym_1146 = ((uint8_t*)"dummy_main_2.a");
    S_23 sym_1147 = sym_82((&(sym_1145)), sym_1146);
    S_59 sym_1148 = sym_311();
    uint32_t sym_1149 = sym_313((&(sym_1148)), (&(sym_1145)), sym_50(sym_1146), sym_1147);
    uint32_t sym_1150 = sym_314((&(sym_1148)), sym_1149, sym_50(((uint8_t*)"main")));
    sym_315((&(sym_1148)), sym_1150);
    sym_316((&(sym_1148)), sym_1150);
    sym_320((&(sym_1148)), sym_1150);
    sym_321((&(sym_1148)), (&(sym_1145)), sym_1150);
    S_23 sym_1151 = sym_337((&((sym_1148)._f542)), sym_82((&(sym_1145)), ((uint8_t*)"prelude.c")));
    sym_83(((uint8_t*)"dummy_out.c"), sym_1151);
    return;
}

S_14 sym_69(uint64_t sym_1153) {
    S_12* sym_1154 = sym_68(sym_1153, ((uint8_t*)0));
    return ((S_14){._f104 = sym_1154, ._f548 = sym_1153});
}

S_12* sym_68(uint64_t sym_1155, uint8_t* sym_1156) {
    S_12* sym_1157 = ((S_12*)malloc(sym_64()));
    uint8_t* sym_1158 = malloc(sym_1155);
    ((*(sym_1157)))._f551 = sym_1156;
    ((*(sym_1157)))._f552 = sym_1158;
    ((*(sym_1157)))._f553 = 0;
    ((*(sym_1157)))._f554 = sym_1155;
    return sym_1157;
}

uint64_t sym_64(void) {
    return 32;
}

S_23 sym_82(S_14* sym_1160, uint8_t* sym_1161) {
    uint8_t* sym_1162 = file_open(sym_1161, ((uint8_t*)"rb"));
    bool sym_1163 = (sym_1162 == ((uint8_t*)0));
    if (sym_1163) {
        return sym_49();
    }
    file_seek(sym_1162, 0, sym_81());
    uint64_t sym_1164 = ((uint64_t)file_tell(sym_1162));
    file_seek(sym_1162, 0, sym_80());
    uint8_t* sym_1165 = sym_70(sym_1160, (sym_1164 + 1));
    uint64_t sym_1166 = file_read(sym_1162, sym_1165, 1, sym_1164);
    file_close(sym_1162);
    (*((sym_1165 + sym_1166))) = ((uint8_t)0);
    return ((S_23){._f566 = sym_1165, ._f567 = sym_1166});
}

S_23 sym_49(void) {
    return ((S_23){._f566 = ((uint8_t*)0), ._f567 = 0});
}

int32_t sym_81(void) {
    return 2;
}

int32_t sym_80(void) {
    return 0;
}

uint8_t* sym_70(S_14* sym_1173, uint64_t sym_1174) {
    uint64_t sym_1175 = sym_67(sym_1174);
    S_12* sym_1176 = ((*(sym_1173)))._f104;
    if (((((*(sym_1176)))._f553 + sym_1175) > ((*(sym_1176)))._f554)) {
        uint64_t sym_1177 = ((*(sym_1173)))._f548;
        if ((sym_1175 > sym_1177)) {
            sym_1177 = sym_1175;
        }
        S_12* sym_1178 = sym_68(sym_1177, ((uint8_t*)sym_1176));
        ((*(sym_1173)))._f104 = sym_1178;
        sym_1176 = sym_1178;
    }
    uint8_t* sym_1179 = (((*(sym_1176)))._f552 + ((*(sym_1176)))._f553);
    ((*(sym_1176)))._f553 = (((*(sym_1176)))._f553 + sym_1175);
    return sym_1179;
}

uint64_t sym_67(uint64_t sym_1180) {
    return ((sym_1180 + 7) & (~(7)));
}

S_59 sym_311(void) {
    return ((S_59){._f579 = sym_215(), ._f580 = 0, ._f542 = sym_327()});
}

S_57 sym_215(void) {
    return ((S_57){._f582 = sym_11(), ._f583 = sym_90(), ._f584 = sym_390(), ._f585 = sym_390(), ._f586 = sym_207(), ._f587 = sym_257(), ._f588 = sym_296(), ._f589 = sym_534(), ._f590 = sym_534(), ._f591 = sym_534(), ._f592 = sym_534(), ._f593 = sym_534(), ._f594 = sym_534(), ._f595 = sym_534(), ._f596 = sym_534(), ._f597 = sym_534(), ._f598 = sym_534(), ._f599 = sym_534()});
}

S_50 sym_11(void) {
    return ((S_50){._f600 = sym_348(), ._f601 = sym_390(), ._f567 = sym_390(), ._f602 = sym_432(), ._f603 = sym_474()});
}

S_42 sym_348(void) {
    return sym_367();
}

S_42 sym_367(void) {
    return sym_368(4);
}

S_42 sym_368(uint64_t sym_1186) {
    uint64_t sym_1187 = sym_1186;
    if ((sym_1187 < 4)) {
        sym_1187 = 4;
    }
    uint8_t* sym_1188 = ((uint8_t*)malloc((sym_1187 * sym_347())));
    return ((S_42){._f566 = sym_1188, ._f567 = 0, ._f554 = sym_1187});
}

uint64_t sym_347(void) {
    return sym_577();
}

uint64_t sym_577(void) {
    return 1;
}

S_45 sym_390(void) {
    return sym_409();
}

S_45 sym_409(void) {
    return sym_410(4);
}

S_45 sym_410(uint64_t sym_1190) {
    uint64_t sym_1191 = sym_1190;
    if ((sym_1191 < 4)) {
        sym_1191 = 4;
    }
    uint32_t* sym_1192 = ((uint32_t*)malloc((sym_1191 * sym_389())));
    return ((S_45){._f566 = sym_1192, ._f567 = 0, ._f554 = sym_1191});
}

uint64_t sym_389(void) {
    return sym_523();
}

uint64_t sym_523(void) {
    return 4;
}

S_47 sym_432(void) {
    return sym_451();
}

S_47 sym_451(void) {
    return sym_452(4);
}

S_47 sym_452(uint64_t sym_1193) {
    uint64_t sym_1194 = sym_1193;
    if ((sym_1194 < 4)) {
        sym_1194 = 4;
    }
    S_23* sym_1195 = ((S_23*)malloc((sym_1194 * sym_431())));
    return ((S_47){._f566 = sym_1195, ._f567 = 0, ._f554 = sym_1194});
}

uint64_t sym_431(void) {
    return sym_47();
}

uint64_t sym_47(void) {
    return 16;
}

S_49 sym_474(void) {
    return sym_493();
}

S_49 sym_493(void) {
    return sym_494(4);
}

S_49 sym_494(uint64_t sym_1196) {
    uint64_t sym_1197 = sym_1196;
    if ((sym_1197 < 4)) {
        sym_1197 = 4;
    }
    bool* sym_1198 = ((bool*)malloc((sym_1197 * sym_473())));
    return ((S_49){._f566 = sym_1198, ._f567 = 0, ._f554 = sym_1197});
}

uint64_t sym_473(void) {
    return sym_589();
}

uint64_t sym_589(void) {
    return 1;
}

S_51 sym_90(void) {
    return ((S_51){._f605 = sym_85(), ._f606 = sym_348(), ._f607 = sym_390(), ._f608 = sym_390(), ._f609 = sym_390()});
}

uint32_t sym_85(void) {
    return sym_524();
}

uint32_t sym_524(void) {
    return 4294967295;
}

S_52 sym_207(void) {
    return sym_208(64);
}

S_52 sym_208(uint64_t sym_1199) {
    uint64_t sym_1200 = sym_1199;
    if ((sym_1200 < 64)) {
        sym_1200 = 64;
    }
    uint64_t sym_1201 = (sym_1200 * sym_523());
    uint32_t* sym_1202 = ((uint32_t*)malloc(sym_1201));
    mem_set(((uint8_t*)sym_1202), ((uint8_t)255), sym_1201);
    return ((S_52){._f610 = sym_432(), ._f611 = sym_1202, ._f554 = sym_1200, ._f578 = 0});
}

S_54 sym_257(void) {
    S_53 sym_1206 = sym_247();
    S_112 sym_1207 = sym_234();
    sym_250((&(sym_1206)), (sym_1207)._f636, 0);
    sym_250((&(sym_1206)), (sym_1207)._f637, 0);
    sym_250((&(sym_1206)), (sym_1207)._f638, 0);
    sym_250((&(sym_1206)), (sym_1207)._f604, 0);
    sym_250((&(sym_1206)), (sym_1207)._f639, 0);
    sym_250((&(sym_1206)), (sym_1207)._f547, 0);
    sym_250((&(sym_1206)), (sym_1207)._f640, 0);
    sym_250((&(sym_1206)), (sym_1207)._f641, 0);
    uint32_t sym_1208 = 64;
    S_45 sym_1209 = sym_391(((uint64_t)sym_1208));
    uint32_t sym_1210 = 0;
    while ((sym_1210 < sym_1208)) {
        sym_394((&(sym_1209)), sym_236());
        sym_1210 = (sym_1210 + 1);
    }
    return ((S_54){._f612 = sym_1206, ._f611 = sym_1209, ._f613 = sym_1208, ._f614 = 0});
}

S_53 sym_247(void) {
    return ((S_53){._f606 = sym_348(), ._f615 = sym_390(), ._f609 = sym_390()});
}

S_112 sym_234(void) {
    return ((S_112){._f636 = 0, ._f645 = 1, ._f637 = 2, ._f638 = 3, ._f646 = 4, ._f647 = 5, ._f648 = 6, ._f604 = 7, ._f566 = 8, ._f639 = 9, ._f547 = 10, ._f649 = 11, ._f640 = 12, ._f650 = 13, ._f651 = 14, ._f641 = 15});
}

uint32_t sym_250(S_53* sym_1211, uint8_t sym_1212, uint32_t sym_1213) {
    uint32_t sym_1214 = ((uint32_t)(((*(sym_1211)))._f606)._f567);
    sym_352((&(((*(sym_1211)))._f606)), sym_1212);
    sym_394((&(((*(sym_1211)))._f615)), sym_1213);
    return sym_1214;
}

void sym_352(S_42* sym_1215, uint8_t sym_1216) {
    sym_372(sym_1215, sym_1216);
    return;
}

void sym_372(S_42* sym_1217, uint8_t sym_1218) {
    sym_371(sym_1217);
    (*((((*(sym_1217)))._f566 + ((*(sym_1217)))._f567))) = sym_1218;
    ((*(sym_1217)))._f567 = (((*(sym_1217)))._f567 + 1);
    return;
}

void sym_371(S_42* sym_1219) {
    if ((((*(sym_1219)))._f567 == ((*(sym_1219)))._f554)) {
        sym_370(sym_1219, 1);
    }
    return;
}

void sym_370(S_42* sym_1220, uint64_t sym_1221) {
    uint64_t sym_1222 = (((*(sym_1220)))._f567 + sym_1221);
    if ((sym_1222 > ((*(sym_1220)))._f554)) {
        uint64_t sym_1223 = (((*(sym_1220)))._f554 * 2);
        if ((sym_1223 < 4)) {
            sym_1223 = 4;
        }
        while ((sym_1223 < sym_1222)) {
            sym_1223 = (sym_1223 * 2);
        }
        uint8_t* sym_1224 = ((uint8_t*)realloc(((uint8_t*)((*(sym_1220)))._f566), (sym_1223 * sym_347())));
        ((*(sym_1220)))._f566 = sym_1224;
        ((*(sym_1220)))._f554 = sym_1223;
    }
    return;
}

void sym_394(S_45* sym_1227, uint32_t sym_1228) {
    sym_414(sym_1227, sym_1228);
    return;
}

void sym_414(S_45* sym_1229, uint32_t sym_1230) {
    sym_413(sym_1229);
    (*((((*(sym_1229)))._f566 + ((*(sym_1229)))._f567))) = sym_1230;
    ((*(sym_1229)))._f567 = (((*(sym_1229)))._f567 + 1);
    return;
}

void sym_413(S_45* sym_1231) {
    if ((((*(sym_1231)))._f567 == ((*(sym_1231)))._f554)) {
        sym_412(sym_1231, 1);
    }
    return;
}

void sym_412(S_45* sym_1232, uint64_t sym_1233) {
    uint64_t sym_1234 = (((*(sym_1232)))._f567 + sym_1233);
    if ((sym_1234 > ((*(sym_1232)))._f554)) {
        uint64_t sym_1235 = (((*(sym_1232)))._f554 * 2);
        if ((sym_1235 < 4)) {
            sym_1235 = 4;
        }
        while ((sym_1235 < sym_1234)) {
            sym_1235 = (sym_1235 * 2);
        }
        uint32_t* sym_1236 = ((uint32_t*)realloc(((uint8_t*)((*(sym_1232)))._f566), (sym_1235 * sym_389())));
        ((*(sym_1232)))._f566 = sym_1236;
        ((*(sym_1232)))._f554 = sym_1235;
    }
    return;
}

S_45 sym_391(uint64_t sym_1237) {
    return sym_410(sym_1237);
}

uint32_t sym_236(void) {
    return sym_524();
}

S_55 sym_296(void) {
    return ((S_55){._f616 = sym_390(), ._f617 = sym_348(), ._f587 = sym_390(), ._f618 = sym_390(), ._f619 = sym_390(), ._f620 = sym_390()});
}

S_56 sym_534(void) {
    return ((S_56){._f621 = sym_390()});
}

S_58 sym_327(void) {
    return ((S_58){._f622 = sym_348(), ._f623 = sym_348(), ._f624 = sym_348(), ._f625 = sym_348(), ._f626 = sym_390(), ._f627 = 0, ._f628 = false, ._f629 = 0});
}

uint32_t sym_313(S_59* sym_1238, S_14* sym_1239, S_23 sym_1240, S_23 sym_1241) {
    uint32_t sym_1242 = ((*(sym_1238)))._f580;
    ((*(sym_1238)))._f580 = (sym_1242 + 1);
    uint64_t sym_1243 = sym_14((&((((*(sym_1238)))._f579)._f582)));
    sym_43((&((((*(sym_1238)))._f579)._f582)), sym_1241);
    uint64_t sym_1244 = sym_14((&((((*(sym_1238)))._f579)._f582)));
    sym_567((&(((*(sym_1238)))._f579)), sym_1239, sym_1242, sym_1240, ((uint32_t)sym_1243), ((uint32_t)sym_1244));
    return sym_1242;
}

uint64_t sym_14(S_50* sym_1245) {
    return (((*(sym_1245)))._f600)._f567;
}

void sym_43(S_50* sym_1246, S_23 sym_1247) {
    S_197 sym_1248 = sym_21(sym_1247);
    while ((!(sym_22((&(sym_1248)))))) {
        uint8_t sym_1249 = sym_24((&(sym_1248)));
        if (sym_32(sym_1249)) {
            sym_34((&(sym_1248)));
            continue;
        }
        if (sym_33(sym_1249)) {
            sym_35((&(sym_1248)));
            continue;
        }
        if (((sym_1249 == 45) && (sym_25((&(sym_1248))) == 45))) {
            sym_36((&(sym_1248)));
            continue;
        }
        if (((sym_27(sym_1249) || ((sym_1249 == 46) && sym_27(sym_25((&(sym_1248)))))) || false)) {
            sym_37((&(sym_1248)), sym_1246);
            continue;
        }
        if ((sym_1249 == 34)) {
            sym_38((&(sym_1248)), sym_1246);
            continue;
        }
        if ((sym_1249 == 39)) {
            sym_39((&(sym_1248)), sym_1246);
            continue;
        }
        if (sym_30(sym_1249)) {
            sym_41((&(sym_1248)), sym_1246);
            continue;
        }
        uint64_t sym_1250 = (sym_1248)._f664;
        bool sym_1251 = (sym_1248)._f666;
        uint8_t sym_1252 = sym_42((&(sym_1248)));
        uint32_t sym_1253 = ((uint32_t)((sym_1248)._f664 - sym_1250));
        S_23 sym_1254 = sym_48((((sym_1248)._f536)._f566 + sym_1250), ((uint64_t)sym_1253));
        sym_13(sym_1246, sym_1252, ((uint32_t)sym_1250), sym_1253, sym_1254, sym_1251);
        (sym_1248)._f666 = false;
    }
    sym_13(sym_1246, (sym_8())._f667, ((uint32_t)(sym_1248)._f664), 0, sym_49(), (sym_1248)._f666);
    return;
}

S_197 sym_21(S_23 sym_1255) {
    return ((S_197){._f536 = sym_1255, ._f664 = 0, ._f666 = true});
}

bool sym_22(S_197* sym_1256) {
    return (((*(sym_1256)))._f664 >= (((*(sym_1256)))._f536)._f567);
}

uint8_t sym_24(S_197* sym_1257) {
    if (sym_22(sym_1257)) {
        return ((uint8_t)0);
    }
    return sym_23(sym_1257, ((*(sym_1257)))._f664);
}

uint8_t sym_23(S_197* sym_1258, uint64_t sym_1259) {
    return (*(((((*(sym_1258)))._f536)._f566 + sym_1259)));
}

bool sym_32(uint8_t sym_1260) {
    return ((sym_1260 == 32) || (sym_1260 == 9));
}

void sym_34(S_197* sym_1261) {
    while ((((!(sym_22(sym_1261))) && sym_32(sym_24(sym_1261))) || false)) {
        sym_26(sym_1261);
    }
    return;
}

void sym_26(S_197* sym_1262) {
    if ((!(sym_22(sym_1262)))) {
        ((*(sym_1262)))._f664 = (((*(sym_1262)))._f664 + 1);
    }
    return;
}

bool sym_33(uint8_t sym_1263) {
    return ((sym_1263 == 10) || (sym_1263 == 13));
}

void sym_35(S_197* sym_1264) {
    while ((((!(sym_22(sym_1264))) && sym_33(sym_24(sym_1264))) || false)) {
        sym_26(sym_1264);
        ((*(sym_1264)))._f666 = true;
    }
    return;
}

uint8_t sym_25(S_197* sym_1265) {
    if (((((*(sym_1265)))._f664 + 1) >= (((*(sym_1265)))._f536)._f567)) {
        return ((uint8_t)0);
    }
    return sym_23(sym_1265, (((*(sym_1265)))._f664 + 1));
}

void sym_36(S_197* sym_1266) {
    sym_26(sym_1266);
    sym_26(sym_1266);
    while ((((!(sym_22(sym_1266))) && (!(sym_33(sym_24(sym_1266))))) || false)) {
        sym_26(sym_1266);
    }
    return;
}

bool sym_27(uint8_t sym_1267) {
    return ((sym_1267 >= 48) && (sym_1267 <= 57));
}

void sym_37(S_197* sym_1268, S_50* sym_1269) {
    uint64_t sym_1270 = ((*(sym_1268)))._f664;
    bool sym_1271 = false;
    if ((sym_24(sym_1268) == 48)) {
        uint8_t sym_1272 = sym_25(sym_1268);
        if (((sym_1272 == 120) || (sym_1272 == 88))) {
            sym_26(sym_1268);
            sym_26(sym_1268);
            while ((((!(sym_22(sym_1268))) && (sym_28(sym_24(sym_1268)) || (sym_24(sym_1268) == 95))) || false)) {
                sym_26(sym_1268);
            }
            uint32_t sym_1273 = ((uint32_t)(((*(sym_1268)))._f664 - sym_1270));
            S_23 sym_1274 = sym_48(((((*(sym_1268)))._f536)._f566 + sym_1270), ((uint64_t)sym_1273));
            sym_13(sym_1269, (sym_8())._f670, ((uint32_t)sym_1270), sym_1273, sym_1274, ((*(sym_1268)))._f666);
            ((*(sym_1268)))._f666 = false;
            return;
        }
    }
    while ((((!(sym_22(sym_1268))) && (sym_27(sym_24(sym_1268)) || (sym_24(sym_1268) == 95))) || false)) {
        sym_26(sym_1268);
    }
    if ((((sym_24(sym_1268) == 46) && (sym_25(sym_1268) != 46)) && sym_27(sym_25(sym_1268)))) {
        sym_1271 = true;
        sym_26(sym_1268);
        while ((((!(sym_22(sym_1268))) && (sym_27(sym_24(sym_1268)) || (sym_24(sym_1268) == 95))) || false)) {
            sym_26(sym_1268);
        }
    }
    uint32_t sym_1275 = ((uint32_t)(((*(sym_1268)))._f664 - sym_1270));
    S_23 sym_1276 = sym_48(((((*(sym_1268)))._f536)._f566 + sym_1270), ((uint64_t)sym_1275));
    uint8_t sym_1277 = (sym_8())._f670;
    if (sym_1271) {
        sym_1277 = (sym_8())._f672;
    }
    sym_13(sym_1269, sym_1277, ((uint32_t)sym_1270), sym_1275, sym_1276, ((*(sym_1268)))._f666);
    ((*(sym_1268)))._f666 = false;
    return;
}

bool sym_28(uint8_t sym_1278) {
    return ((((sym_1278 >= 48) && (sym_1278 <= 57)) || ((sym_1278 >= 65) && (sym_1278 <= 70))) || ((sym_1278 >= 97) && (sym_1278 <= 102)));
}

S_23 sym_48(uint8_t* sym_1279, uint64_t sym_1280) {
    return ((S_23){._f566 = sym_1279, ._f567 = sym_1280});
}

void sym_13(S_50* sym_1281, uint8_t sym_1282, uint32_t sym_1283, uint32_t sym_1284, S_23 sym_1285, bool sym_1286) {
    sym_352((&(((*(sym_1281)))._f600)), sym_1282);
    sym_394((&(((*(sym_1281)))._f601)), sym_1283);
    sym_394((&(((*(sym_1281)))._f567)), sym_1284);
    sym_436((&(((*(sym_1281)))._f602)), sym_1285);
    sym_478((&(((*(sym_1281)))._f603)), sym_1286);
    return;
}

void sym_436(S_47* sym_1287, S_23 sym_1288) {
    sym_456(sym_1287, sym_1288);
    return;
}

void sym_456(S_47* sym_1289, S_23 sym_1290) {
    sym_455(sym_1289);
    (*((((*(sym_1289)))._f566 + ((*(sym_1289)))._f567))) = sym_1290;
    ((*(sym_1289)))._f567 = (((*(sym_1289)))._f567 + 1);
    return;
}

void sym_455(S_47* sym_1291) {
    if ((((*(sym_1291)))._f567 == ((*(sym_1291)))._f554)) {
        sym_454(sym_1291, 1);
    }
    return;
}

void sym_454(S_47* sym_1292, uint64_t sym_1293) {
    uint64_t sym_1294 = (((*(sym_1292)))._f567 + sym_1293);
    if ((sym_1294 > ((*(sym_1292)))._f554)) {
        uint64_t sym_1295 = (((*(sym_1292)))._f554 * 2);
        if ((sym_1295 < 4)) {
            sym_1295 = 4;
        }
        while ((sym_1295 < sym_1294)) {
            sym_1295 = (sym_1295 * 2);
        }
        S_23* sym_1296 = ((S_23*)realloc(((uint8_t*)((*(sym_1292)))._f566), (sym_1295 * sym_431())));
        ((*(sym_1292)))._f566 = sym_1296;
        ((*(sym_1292)))._f554 = sym_1295;
    }
    return;
}

void sym_478(S_49* sym_1297, bool sym_1298) {
    sym_498(sym_1297, sym_1298);
    return;
}

void sym_498(S_49* sym_1299, bool sym_1300) {
    sym_497(sym_1299);
    (*((((*(sym_1299)))._f566 + ((*(sym_1299)))._f567))) = sym_1300;
    ((*(sym_1299)))._f567 = (((*(sym_1299)))._f567 + 1);
    return;
}

void sym_497(S_49* sym_1301) {
    if ((((*(sym_1301)))._f567 == ((*(sym_1301)))._f554)) {
        sym_496(sym_1301, 1);
    }
    return;
}

void sym_496(S_49* sym_1302, uint64_t sym_1303) {
    uint64_t sym_1304 = (((*(sym_1302)))._f567 + sym_1303);
    if ((sym_1304 > ((*(sym_1302)))._f554)) {
        uint64_t sym_1305 = (((*(sym_1302)))._f554 * 2);
        if ((sym_1305 < 4)) {
            sym_1305 = 4;
        }
        while ((sym_1305 < sym_1304)) {
            sym_1305 = (sym_1305 * 2);
        }
        bool* sym_1306 = ((bool*)realloc(((uint8_t*)((*(sym_1302)))._f566), (sym_1305 * sym_473())));
        ((*(sym_1302)))._f566 = sym_1306;
        ((*(sym_1302)))._f554 = sym_1305;
    }
    return;
}

S_222 sym_8(void) {
    return ((S_222){._f667 = 0, ._f636 = 1, ._f673 = 2, ._f670 = 3, ._f672 = 4, ._f674 = 5, ._f675 = 6, ._f676 = 7, ._f677 = 8, ._f678 = 9, ._f679 = 10, ._f680 = 11, ._f681 = 12, ._f682 = 13, ._f683 = 14, ._f684 = 15, ._f685 = 16, ._f686 = 17, ._f687 = 18, ._f688 = 19, ._f689 = 20, ._f690 = 21, ._f691 = 22, ._f692 = 23, ._f693 = 24, ._f694 = 25, ._f695 = 26, ._f696 = 27, ._f697 = 28, ._f698 = 29, ._f699 = 30, ._f700 = 31, ._f701 = 32, ._f702 = 33, ._f703 = 34, ._f704 = 35, ._f705 = 36, ._f706 = 37, ._f707 = 38, ._f708 = 39, ._f709 = 40, ._f710 = 41, ._f711 = 42, ._f712 = 43, ._f713 = 44, ._f714 = 45, ._f715 = 46, ._f716 = 47, ._f717 = 48, ._f718 = 49, ._f719 = 50, ._f720 = 51, ._f721 = 52, ._f722 = 53, ._f723 = 54, ._f724 = 55, ._f725 = 56, ._f726 = 57, ._f727 = 58, ._f728 = 59, ._f729 = 60, ._f730 = 61});
}

void sym_38(S_197* sym_1307, S_50* sym_1308) {
    uint64_t sym_1309 = ((*(sym_1307)))._f664;
    sym_26(sym_1307);
    while (((!(sym_22(sym_1307))) && (sym_24(sym_1307) != 34))) {
        if ((sym_24(sym_1307) == 92)) {
            sym_26(sym_1307);
        }
        sym_26(sym_1307);
    }
    if ((!(sym_22(sym_1307)))) {
        sym_26(sym_1307);
    }
    uint32_t sym_1310 = ((uint32_t)(((*(sym_1307)))._f664 - sym_1309));
    S_23 sym_1311 = sym_48(((((*(sym_1307)))._f536)._f566 + sym_1309), ((uint64_t)sym_1310));
    sym_13(sym_1308, (sym_8())._f674, ((uint32_t)sym_1309), sym_1310, sym_1311, ((*(sym_1307)))._f666);
    ((*(sym_1307)))._f666 = false;
    return;
}

void sym_39(S_197* sym_1312, S_50* sym_1313) {
    uint64_t sym_1314 = ((*(sym_1312)))._f664;
    sym_26(sym_1312);
    if ((sym_24(sym_1312) == 92)) {
        sym_26(sym_1312);
    }
    sym_26(sym_1312);
    if ((sym_24(sym_1312) == 39)) {
        sym_26(sym_1312);
    }
    uint32_t sym_1315 = ((uint32_t)(((*(sym_1312)))._f664 - sym_1314));
    S_23 sym_1316 = sym_48(((((*(sym_1312)))._f536)._f566 + sym_1314), ((uint64_t)sym_1315));
    sym_13(sym_1313, (sym_8())._f675, ((uint32_t)sym_1314), sym_1315, sym_1316, ((*(sym_1312)))._f666);
    ((*(sym_1312)))._f666 = false;
    return;
}

bool sym_30(uint8_t sym_1317) {
    return sym_29(sym_1317);
}

bool sym_29(uint8_t sym_1318) {
    return ((((sym_1318 >= 65) && (sym_1318 <= 90)) || ((sym_1318 >= 97) && (sym_1318 <= 122))) || (sym_1318 == 95));
}

void sym_41(S_197* sym_1319, S_50* sym_1320) {
    uint64_t sym_1321 = ((*(sym_1319)))._f664;
    while ((((!(sym_22(sym_1319))) && sym_31(sym_24(sym_1319))) || false)) {
        sym_26(sym_1319);
    }
    uint32_t sym_1322 = ((uint32_t)(((*(sym_1319)))._f664 - sym_1321));
    S_23 sym_1323 = sym_48(((((*(sym_1319)))._f536)._f566 + sym_1321), ((uint64_t)sym_1322));
    uint8_t sym_1324 = sym_40(sym_1323);
    sym_13(sym_1320, sym_1324, ((uint32_t)sym_1321), sym_1322, sym_1323, ((*(sym_1319)))._f666);
    ((*(sym_1319)))._f666 = false;
    return;
}

bool sym_31(uint8_t sym_1325) {
    return (sym_29(sym_1325) || sym_27(sym_1325));
}

uint8_t sym_40(S_23 sym_1326) {
    S_222 sym_1327 = sym_8();
    if (((sym_1326)._f567 == 2)) {
        if (sym_54(sym_1326, sym_50(((uint8_t*)"if")))) {
            return (sym_1327)._f677;
        }
        if (sym_54(sym_1326, sym_50(((uint8_t*)"as")))) {
            return (sym_1327)._f687;
        }
        if (sym_54(sym_1326, sym_50(((uint8_t*)"or")))) {
            return (sym_1327)._f689;
        }
    }
    if (((sym_1326)._f567 == 3)) {
        if (sym_54(sym_1326, sym_50(((uint8_t*)"ret")))) {
            return (sym_1327)._f676;
        }
        if (sym_54(sym_1326, sym_50(((uint8_t*)"and")))) {
            return (sym_1327)._f688;
        }
        if (sym_54(sym_1326, sym_50(((uint8_t*)"not")))) {
            return (sym_1327)._f690;
        }
    }
    if (((sym_1326)._f567 == 4)) {
        if (sym_54(sym_1326, sym_50(((uint8_t*)"else")))) {
            return (sym_1327)._f678;
        }
        if (sym_54(sym_1326, sym_50(((uint8_t*)"cont")))) {
            return (sym_1327)._f681;
        }
        if (sym_54(sym_1326, sym_50(((uint8_t*)"true")))) {
            return (sym_1327)._f685;
        }
        if (sym_54(sym_1326, sym_50(((uint8_t*)"from")))) {
            return (sym_1327)._f684;
        }
    }
    if (((sym_1326)._f567 == 5)) {
        if (sym_54(sym_1326, sym_50(((uint8_t*)"while")))) {
            return (sym_1327)._f679;
        }
        if (sym_54(sym_1326, sym_50(((uint8_t*)"break")))) {
            return (sym_1327)._f680;
        }
        if (sym_54(sym_1326, sym_50(((uint8_t*)"false")))) {
            return (sym_1327)._f686;
        }
    }
    if (((sym_1326)._f567 == 6)) {
        if (sym_54(sym_1326, sym_50(((uint8_t*)"struct")))) {
            return (sym_1327)._f682;
        }
        if (sym_54(sym_1326, sym_50(((uint8_t*)"import")))) {
            return (sym_1327)._f683;
        }
    }
    return (sym_1327)._f673;
}

bool sym_54(S_23 sym_1328, S_23 sym_1329) {
    if (((sym_1328)._f567 != (sym_1329)._f567)) {
        return false;
    }
    uint64_t sym_1330 = 0;
    while ((sym_1330 < (sym_1328)._f567)) {
        if ((((*(((sym_1328)._f566 + sym_1330))) != (*(((sym_1329)._f566 + sym_1330)))) || false)) {
            return false;
        }
        sym_1330 = (sym_1330 + 1);
    }
    return true;
}

S_23 sym_50(uint8_t* sym_1331) {
    return ((S_23){._f566 = sym_1331, ._f567 = str_len(sym_1331)});
}

uint8_t sym_42(S_197* sym_1333) {
    S_222 sym_1334 = sym_8();
    uint8_t sym_1335 = sym_24(sym_1333);
    sym_26(sym_1333);
    if ((sym_1335 == 43)) {
        if ((sym_24(sym_1333) == 61)) {
            sym_26(sym_1333);
            return (sym_1334)._f696;
        }
        return (sym_1334)._f691;
    }
    if ((sym_1335 == 45)) {
        if ((sym_24(sym_1333) == 61)) {
            sym_26(sym_1333);
            return (sym_1334)._f697;
        }
        return (sym_1334)._f692;
    }
    if ((sym_1335 == 42)) {
        if ((sym_24(sym_1333) == 61)) {
            sym_26(sym_1333);
            return (sym_1334)._f698;
        }
        return (sym_1334)._f693;
    }
    if ((sym_1335 == 47)) {
        if ((sym_24(sym_1333) == 61)) {
            sym_26(sym_1333);
            return (sym_1334)._f699;
        }
        return (sym_1334)._f694;
    }
    if ((sym_1335 == 37)) {
        if ((sym_24(sym_1333) == 61)) {
            sym_26(sym_1333);
            return (sym_1334)._f700;
        }
        return (sym_1334)._f695;
    }
    if ((sym_1335 == 38)) {
        if ((sym_24(sym_1333) == 61)) {
            sym_26(sym_1333);
            return (sym_1334)._f705;
        }
        return (sym_1334)._f701;
    }
    if ((sym_1335 == 124)) {
        if ((sym_24(sym_1333) == 61)) {
            sym_26(sym_1333);
            return (sym_1334)._f706;
        }
        return (sym_1334)._f702;
    }
    if ((sym_1335 == 94)) {
        if ((sym_24(sym_1333) == 61)) {
            sym_26(sym_1333);
            return (sym_1334)._f707;
        }
        return (sym_1334)._f703;
    }
    if ((sym_1335 == 126)) {
        return (sym_1334)._f704;
    }
    if ((sym_1335 == 61)) {
        if ((sym_24(sym_1333) == 61)) {
            sym_26(sym_1333);
            return (sym_1334)._f713;
        }
        return (sym_1334)._f712;
    }
    if ((sym_1335 == 33)) {
        if ((sym_24(sym_1333) == 61)) {
            sym_26(sym_1333);
            return (sym_1334)._f714;
        }
        return (sym_1334)._f636;
    }
    if ((sym_1335 == 58)) {
        if ((sym_24(sym_1333) == 58)) {
            sym_26(sym_1333);
            return (sym_1334)._f720;
        }
        return (sym_1334)._f719;
    }
    if ((sym_1335 == 46)) {
        return (sym_1334)._f723;
    }
    if ((sym_1335 == 60)) {
        if ((sym_24(sym_1333) == 60)) {
            sym_26(sym_1333);
            if ((sym_24(sym_1333) == 61)) {
                sym_26(sym_1333);
                return (sym_1334)._f710;
            }
            return (sym_1334)._f708;
        }
        if ((sym_24(sym_1333) == 61)) {
            sym_26(sym_1333);
            return (sym_1334)._f716;
        }
        return (sym_1334)._f715;
    }
    if ((sym_1335 == 62)) {
        if ((sym_24(sym_1333) == 62)) {
            sym_26(sym_1333);
            if ((sym_24(sym_1333) == 61)) {
                sym_26(sym_1333);
                return (sym_1334)._f711;
            }
            return (sym_1334)._f709;
        }
        if ((sym_24(sym_1333) == 61)) {
            sym_26(sym_1333);
            return (sym_1334)._f718;
        }
        return (sym_1334)._f717;
    }
    if ((sym_1335 == 44)) {
        return (sym_1334)._f722;
    }
    if ((sym_1335 == 59)) {
        return (sym_1334)._f721;
    }
    if ((sym_1335 == 40)) {
        return (sym_1334)._f725;
    }
    if ((sym_1335 == 41)) {
        return (sym_1334)._f726;
    }
    if ((sym_1335 == 123)) {
        return (sym_1334)._f727;
    }
    if ((sym_1335 == 125)) {
        return (sym_1334)._f728;
    }
    if ((sym_1335 == 91)) {
        return (sym_1334)._f729;
    }
    if ((sym_1335 == 93)) {
        return (sym_1334)._f730;
    }
    if ((sym_1335 == 35)) {
        return (sym_1334)._f724;
    }
    return (sym_1334)._f636;
}

void sym_567(S_57* sym_1336, S_14* sym_1337, uint32_t sym_1338, S_23 sym_1339, uint32_t sym_1340, uint32_t sym_1341) {
    S_45 sym_1342 = sym_390();
    S_45 sym_1343 = sym_390();
    S_45 sym_1344 = sym_390();
    S_45 sym_1345 = sym_390();
    S_45 sym_1346 = sym_390();
    uint32_t sym_1347 = sym_211((&(((*(sym_1336)))._f586)), sym_1339);
    sym_394((&(sym_1342)), sym_1347);
    sym_394((&(sym_1343)), sym_1338);
    sym_394((&(sym_1344)), sym_1340);
    sym_394((&(sym_1345)), sym_1341);
    uint64_t sym_1348 = 0;
    while ((sym_1348 < (sym_1343)._f567)) {
        uint32_t sym_1349 = sym_396((&(sym_1343)), sym_1348);
        uint32_t sym_1350 = sym_396((&(sym_1344)), sym_1348);
        uint32_t sym_1351 = sym_396((&(sym_1345)), sym_1348);
        uint32_t sym_1352 = sym_396((&(sym_1342)), ((uint64_t)sym_1349));
        S_23 sym_1353 = sym_212((&(((*(sym_1336)))._f586)), sym_1352);
        sym_566(sym_1336, sym_1337, sym_1349, sym_1353, sym_1350, sym_1351, (&(sym_1342)), (&(sym_1343)), (&(sym_1344)), (&(sym_1345)), (&(sym_1346)));
        sym_1348 = (sym_1348 + 1);
    }
    bool sym_1354 = true;
    while (sym_1354) {
        sym_1354 = false;
        uint64_t sym_1355 = 0;
        while ((sym_1355 < (sym_1346)._f567)) {
            uint32_t sym_1356 = sym_396((&(sym_1346)), sym_1355);
            if ((sym_1356 != sym_532())) {
                uint32_t sym_1357 = sym_396((&(sym_1346)), (sym_1355 + 1));
                uint32_t sym_1358 = sym_396((&(sym_1346)), (sym_1355 + 2));
                uint32_t sym_1359 = sym_396((&(sym_1346)), (sym_1355 + 3));
                uint32_t sym_1360 = sym_221(sym_1336, sym_1359, sym_1358);
                if ((sym_1360 != sym_532())) {
                    sym_220(sym_1336, sym_1356, sym_1357, sym_1360);
                    sym_397((&(sym_1346)), sym_1355, sym_532());
                    sym_1354 = true;
                }
            }
            sym_1355 = (sym_1355 + 4);
        }
    }
    sym_392((&(sym_1342)));
    sym_392((&(sym_1343)));
    sym_392((&(sym_1344)));
    sym_392((&(sym_1345)));
    sym_392((&(sym_1346)));
    return;
}

uint32_t sym_211(S_52* sym_1361, S_23 sym_1362) {
    if ((((((*(sym_1361)))._f578 + 1) * 10) >= (((*(sym_1361)))._f554 * 7))) {
        sym_210(sym_1361);
    }
    uint64_t sym_1363 = (((*(sym_1361)))._f554 - 1);
    uint64_t sym_1364 = ((uint64_t)sym_561(sym_1362));
    uint64_t sym_1365 = (sym_1364 & sym_1363);
    uint32_t sym_1366 = sym_205();
    while (true) {
        uint32_t sym_1367 = (*((((*(sym_1361)))._f611 + sym_1365)));
        if ((sym_1367 == sym_1366)) {
            uint32_t sym_1368 = ((uint32_t)(((*(sym_1361)))._f610)._f567);
            sym_436((&(((*(sym_1361)))._f610)), sym_1362);
            (*((((*(sym_1361)))._f611 + sym_1365))) = sym_1368;
            ((*(sym_1361)))._f578 = (((*(sym_1361)))._f578 + 1);
            return sym_1368;
        }
        S_23 sym_1369 = sym_438((&(((*(sym_1361)))._f610)), ((uint64_t)sym_1367));
        if (sym_54(sym_1362, sym_1369)) {
            return sym_1367;
        }
        sym_1365 = ((sym_1365 + 1) & sym_1363);
    }
    return sym_1366;
}

void sym_210(S_52* sym_1370) {
    uint64_t sym_1371 = ((*(sym_1370)))._f554;
    uint32_t* sym_1372 = ((*(sym_1370)))._f611;
    uint64_t sym_1373 = (sym_1371 * 2);
    uint64_t sym_1374 = (sym_1373 * sym_523());
    uint32_t* sym_1375 = ((uint32_t*)malloc(sym_1374));
    mem_set(((uint8_t*)sym_1375), ((uint8_t)255), sym_1374);
    uint64_t sym_1376 = (sym_1373 - 1);
    uint32_t sym_1377 = sym_205();
    uint64_t sym_1378 = 0;
    while ((sym_1378 < sym_1371)) {
        uint32_t sym_1379 = (*((sym_1372 + sym_1378)));
        if ((sym_1379 != sym_1377)) {
            S_23 sym_1380 = sym_438((&(((*(sym_1370)))._f610)), ((uint64_t)sym_1379));
            uint64_t sym_1381 = (((uint64_t)sym_561(sym_1380)) & sym_1376);
            while (((*((sym_1375 + sym_1381))) != sym_1377)) {
                sym_1381 = ((sym_1381 + 1) & sym_1376);
            }
            (*((sym_1375 + sym_1381))) = sym_1379;
        }
        sym_1378 = (sym_1378 + 1);
    }
    free(((uint8_t*)sym_1372));
    ((*(sym_1370)))._f611 = sym_1375;
    ((*(sym_1370)))._f554 = sym_1373;
    return;
}

uint32_t sym_205(void) {
    return sym_524();
}

S_23 sym_438(S_47* sym_1382, uint64_t sym_1383) {
    return sym_458(sym_1382, sym_1383);
}

S_23 sym_458(S_47* sym_1384, uint64_t sym_1385) {
    return (*((((*(sym_1384)))._f566 + sym_1385)));
}

uint32_t sym_561(S_23 sym_1386) {
    uint32_t sym_1387 = ((uint32_t)2166136261);
    uint64_t sym_1388 = 0;
    while ((sym_1388 < (sym_1386)._f567)) {
        uint32_t sym_1389 = ((uint32_t)(*(((sym_1386)._f566 + sym_1388))));
        sym_1387 = ((sym_1387 ^ sym_1389) * ((uint32_t)16777619));
        sym_1388 = (sym_1388 + 1);
    }
    return sym_1387;
}

uint32_t sym_396(S_45* sym_1391, uint64_t sym_1392) {
    return sym_416(sym_1391, sym_1392);
}

uint32_t sym_416(S_45* sym_1393, uint64_t sym_1394) {
    return (*((((*(sym_1393)))._f566 + sym_1394)));
}

S_23 sym_212(S_52* sym_1395, uint32_t sym_1396) {
    return sym_438((&(((*(sym_1395)))._f610)), ((uint64_t)sym_1396));
}

void sym_566(S_57* sym_1397, S_14* sym_1398, uint32_t sym_1399, S_23 sym_1400, uint32_t sym_1401, uint32_t sym_1402, S_45* sym_1403, S_45* sym_1404, S_45* sym_1405, S_45* sym_1406, S_45* sym_1407) {
    S_222 sym_1408 = sym_8();
    uint64_t sym_1409 = sym_1401;
    uint64_t sym_1410 = 0;
    bool sym_1411 = false;
    uint32_t sym_1412 = 0;
    uint64_t sym_1413 = 0;
    while ((sym_1409 < sym_1402)) {
        uint8_t sym_1414 = sym_15((&(((*(sym_1397)))._f582)), sym_1409);
        S_23 sym_1415 = sym_18((&(((*(sym_1397)))._f582)), sym_1409);
        if (((sym_1414 == (sym_1408)._f727) || (sym_1414 == (sym_1408)._f725))) {
            sym_1410 = (sym_1410 + 1);
        } else {
            if (((sym_1414 == (sym_1408)._f728) || (sym_1414 == (sym_1408)._f726))) {
                if ((sym_1410 > 0)) {
                    sym_1410 = (sym_1410 - 1);
                }
            } else {
                if (((sym_1410 == 0) && sym_562(sym_1415, sym_50(((uint8_t*)"import"))))) {
                    if (sym_1411) {
                        sym_218(sym_1397, sym_1399, sym_1412, sym_1413, sym_1409);
                        sym_1411 = false;
                    }
                    uint64_t sym_1416 = (sym_1409 + 1);
                    S_45 sym_1417 = sym_390();
                    while ((sym_1416 < sym_1402)) {
                        S_23 sym_1418 = sym_18((&(((*(sym_1397)))._f582)), sym_1416);
                        if (sym_562(sym_1418, sym_50(((uint8_t*)"from")))) {
                            sym_1416 = (sym_1416 + 1);
                            break;
                        }
                        if ((!(sym_562(sym_1418, sym_50(((uint8_t*)",")))))) {
                            uint32_t sym_1419 = sym_211((&(((*(sym_1397)))._f586)), sym_1418);
                            sym_394((&(sym_1417)), sym_1419);
                        }
                        sym_1416 = (sym_1416 + 1);
                    }
                    if ((sym_1416 < sym_1402)) {
                        S_23 sym_1420 = sym_18((&(((*(sym_1397)))._f582)), sym_1416);
                        S_23 sym_1421 = sym_563(sym_1420);
                        uint32_t sym_1422 = sym_565(sym_1397, sym_1398, sym_1400, sym_1421, sym_1403, sym_1404, sym_1405, sym_1406);
                        uint64_t sym_1423 = 0;
                        while ((sym_1423 < (sym_1417)._f567)) {
                            uint32_t sym_1424 = sym_396((&(sym_1417)), sym_1423);
                            sym_394(sym_1407, sym_1399);
                            sym_394(sym_1407, sym_1424);
                            sym_394(sym_1407, sym_1424);
                            sym_394(sym_1407, sym_1422);
                            sym_1423 = (sym_1423 + 1);
                        }
                        sym_392((&(sym_1417)));
                        sym_1409 = (sym_1416 + 1);
                        continue;
                    }
                    sym_392((&(sym_1417)));
                } else {
                    if (((sym_1410 == 0) && (sym_1414 == (sym_1408)._f673))) {
                        if (((sym_1409 + 1) < sym_1402)) {
                            uint8_t sym_1425 = sym_15((&(((*(sym_1397)))._f582)), (sym_1409 + 1));
                            if ((sym_1425 == (sym_1408)._f720)) {
                                if (((sym_1409 + 2) < sym_1402)) {
                                    S_23 sym_1426 = sym_18((&(((*(sym_1397)))._f582)), (sym_1409 + 2));
                                    if (sym_562(sym_1426, sym_50(((uint8_t*)"import")))) {
                                        if (sym_1411) {
                                            sym_218(sym_1397, sym_1399, sym_1412, sym_1413, sym_1409);
                                            sym_1411 = false;
                                        }
                                        S_23 sym_1427 = sym_18((&(((*(sym_1397)))._f582)), sym_1409);
                                        uint32_t sym_1428 = sym_211((&(((*(sym_1397)))._f586)), sym_1427);
                                        S_23 sym_1429 = sym_18((&(((*(sym_1397)))._f582)), (sym_1409 + 3));
                                        uint32_t sym_1430 = sym_211((&(((*(sym_1397)))._f586)), sym_1429);
                                        S_23 sym_1431 = sym_18((&(((*(sym_1397)))._f582)), (sym_1409 + 5));
                                        S_23 sym_1432 = sym_563(sym_1431);
                                        uint32_t sym_1433 = sym_565(sym_1397, sym_1398, sym_1400, sym_1432, sym_1403, sym_1404, sym_1405, sym_1406);
                                        sym_394(sym_1407, sym_1399);
                                        sym_394(sym_1407, sym_1428);
                                        sym_394(sym_1407, sym_1430);
                                        sym_394(sym_1407, sym_1433);
                                        sym_1409 = (sym_1409 + 6);
                                        continue;
                                    }
                                }
                                if (sym_1411) {
                                    sym_218(sym_1397, sym_1399, sym_1412, sym_1413, sym_1409);
                                }
                                S_23 sym_1434 = sym_18((&(((*(sym_1397)))._f582)), sym_1409);
                                sym_1412 = sym_211((&(((*(sym_1397)))._f586)), sym_1434);
                                sym_1413 = sym_1409;
                                sym_1411 = true;
                                sym_1409 = (sym_1409 + 1);
                            }
                        }
                    }
                }
            }
        }
        sym_1409 = (sym_1409 + 1);
    }
    if (sym_1411) {
        sym_218(sym_1397, sym_1399, sym_1412, sym_1413, sym_1402);
    }
    return;
}

uint8_t sym_15(S_50* sym_1435, uint32_t sym_1436) {
    return sym_354((&(((*(sym_1435)))._f600)), ((uint64_t)sym_1436));
}

uint8_t sym_354(S_42* sym_1437, uint64_t sym_1438) {
    return sym_374(sym_1437, sym_1438);
}

uint8_t sym_374(S_42* sym_1439, uint64_t sym_1440) {
    return (*((((*(sym_1439)))._f566 + sym_1440)));
}

S_23 sym_18(S_50* sym_1441, uint32_t sym_1442) {
    return sym_438((&(((*(sym_1441)))._f602)), ((uint64_t)sym_1442));
}

bool sym_562(S_23 sym_1443, S_23 sym_1444) {
    if (((sym_1443)._f567 != (sym_1444)._f567)) {
        return false;
    }
    uint64_t sym_1445 = 0;
    while ((sym_1445 < (sym_1443)._f567)) {
        if (((*(((sym_1443)._f566 + sym_1445))) != (*(((sym_1444)._f566 + sym_1445))))) {
            return false;
        }
        sym_1445 = (sym_1445 + 1);
    }
    return true;
}

uint32_t sym_218(S_57* sym_1446, uint32_t sym_1447, uint32_t sym_1448, uint32_t sym_1449, uint32_t sym_1450) {
    uint32_t sym_1451 = sym_537((&(((*(sym_1446)))._f589)), sym_1448);
    sym_537((&(((*(sym_1446)))._f590)), sym_1447);
    sym_537((&(((*(sym_1446)))._f591)), sym_1449);
    sym_537((&(((*(sym_1446)))._f592)), sym_1450);
    sym_537((&(((*(sym_1446)))._f593)), sym_532());
    sym_537((&(((*(sym_1446)))._f594)), sym_532());
    sym_537((&(((*(sym_1446)))._f596)), sym_532());
    sym_537((&(((*(sym_1446)))._f599)), 0);
    sym_537((&(((*(sym_1446)))._f595)), sym_532());
    return sym_1451;
}

uint32_t sym_537(S_56* sym_1452, uint32_t sym_1453) {
    uint32_t sym_1454 = ((uint32_t)(((*(sym_1452)))._f621)._f567);
    sym_394((&(((*(sym_1452)))._f621)), sym_1453);
    return sym_1454;
}

uint32_t sym_532(void) {
    return sym_524();
}

S_23 sym_563(S_23 sym_1455) {
    if (((sym_1455)._f567 >= 2)) {
        uint8_t sym_1456 = (*((sym_1455)._f566));
        uint8_t sym_1457 = (*((((sym_1455)._f566 + (sym_1455)._f567) - 1)));
        if (((sym_1456 == ((uint8_t)34)) && (sym_1457 == ((uint8_t)34)))) {
            return ((S_23){._f566 = ((sym_1455)._f566 + 1), ._f567 = ((sym_1455)._f567 - 2)});
        }
    }
    return sym_1455;
}

uint32_t sym_565(S_57* sym_1458, S_14* sym_1459, S_23 sym_1460, S_23 sym_1461, S_45* sym_1462, S_45* sym_1463, S_45* sym_1464, S_45* sym_1465) {
    S_23 sym_1466 = sym_564(sym_1459, sym_1460, sym_1461);
    uint32_t sym_1467 = sym_211((&(((*(sym_1458)))._f586)), sym_1466);
    uint64_t sym_1468 = 0;
    while ((sym_1468 < ((*(sym_1462)))._f567)) {
        if ((sym_396(sym_1462, sym_1468) == sym_1467)) {
            return ((uint32_t)sym_1468);
        }
        sym_1468 = (sym_1468 + 1);
    }
    uint32_t sym_1469 = ((uint32_t)((*(sym_1462)))._f567);
    sym_394(sym_1462, sym_1467);
    S_23 sym_1470 = sym_82(sym_1459, (sym_1466)._f566);
    uint32_t sym_1471 = ((uint32_t)sym_14((&(((*(sym_1458)))._f582))));
    sym_43((&(((*(sym_1458)))._f582)), sym_1470);
    uint32_t sym_1472 = ((uint32_t)sym_14((&(((*(sym_1458)))._f582))));
    sym_394(sym_1463, sym_1469);
    sym_394(sym_1464, sym_1471);
    sym_394(sym_1465, sym_1472);
    return sym_1469;
}

S_23 sym_564(S_14* sym_1473, S_23 sym_1474, S_23 sym_1475) {
    uint64_t sym_1476 = 0;
    if (((sym_1474)._f567 > 0)) {
        uint64_t sym_1477 = (sym_1474)._f567;
        while ((sym_1477 > 0)) {
            sym_1477 = (sym_1477 - 1);
            if (((*(((sym_1474)._f566 + sym_1477))) == ((uint8_t)47))) {
                sym_1476 = sym_1477;
                break;
            }
        }
    }
    uint64_t sym_1478 = (sym_1475)._f567;
    if ((sym_1476 > 0)) {
        sym_1478 = ((sym_1476 + 1) + (sym_1475)._f567);
    }
    uint8_t* sym_1479 = sym_70(sym_1473, (sym_1478 + 1));
    if ((sym_1476 > 0)) {
        uint64_t sym_1480 = 0;
        while ((sym_1480 < sym_1476)) {
            (*((sym_1479 + sym_1480))) = (*(((sym_1474)._f566 + sym_1480)));
            sym_1480 = (sym_1480 + 1);
        }
        (*((sym_1479 + sym_1476))) = ((uint8_t)47);
        uint64_t sym_1481 = 0;
        while ((sym_1481 < (sym_1475)._f567)) {
            (*((((sym_1479 + sym_1476) + 1) + sym_1481))) = (*(((sym_1475)._f566 + sym_1481)));
            sym_1481 = (sym_1481 + 1);
        }
    } else {
        uint64_t sym_1482 = 0;
        while ((sym_1482 < (sym_1475)._f567)) {
            (*((sym_1479 + sym_1482))) = (*(((sym_1475)._f566 + sym_1482)));
            sym_1482 = (sym_1482 + 1);
        }
    }
    S_23 sym_1483 = ((S_23){._f566 = sym_1479, ._f567 = sym_1478});
    S_45 sym_1484 = sym_390();
    S_45 sym_1485 = sym_390();
    uint64_t sym_1486 = 0;
    uint64_t sym_1487 = 0;
    while ((sym_1486 <= (sym_1483)._f567)) {
        bool sym_1488 = (sym_1486 == (sym_1483)._f567);
        bool sym_1489 = false;
        if ((!(sym_1488))) {
            if (((*(((sym_1483)._f566 + sym_1486))) == ((uint8_t)47))) {
                sym_1489 = true;
            }
        }
        if ((sym_1488 || sym_1489)) {
            uint64_t sym_1490 = (sym_1486 - sym_1487);
            if ((sym_1490 > 0)) {
                bool sym_1491 = ((sym_1490 == 1) && ((*(((sym_1483)._f566 + sym_1487))) == ((uint8_t)46)));
                bool sym_1492 = (((sym_1490 == 2) && ((*(((sym_1483)._f566 + sym_1487))) == ((uint8_t)46))) && ((*((((sym_1483)._f566 + sym_1487) + 1))) == ((uint8_t)46)));
                if (sym_1491) {
                } else {
                    if (sym_1492) {
                        if (((sym_1484)._f567 > 0)) {
                            uint64_t sym_1493 = ((sym_1484)._f567 - 1);
                            uint64_t sym_1494 = ((uint64_t)sym_396((&(sym_1484)), sym_1493));
                            uint64_t sym_1495 = ((uint64_t)sym_396((&(sym_1485)), sym_1493));
                            bool sym_1496 = (((sym_1495 == 2) && ((*(((sym_1483)._f566 + sym_1494))) == ((uint8_t)46))) && ((*((((sym_1483)._f566 + sym_1494) + 1))) == ((uint8_t)46)));
                            if (sym_1496) {
                                sym_394((&(sym_1484)), ((uint32_t)sym_1487));
                                sym_394((&(sym_1485)), ((uint32_t)sym_1490));
                            } else {
                                (sym_1484)._f567 = ((sym_1484)._f567 - 1);
                                (sym_1485)._f567 = ((sym_1485)._f567 - 1);
                            }
                        } else {
                            sym_394((&(sym_1484)), ((uint32_t)sym_1487));
                            sym_394((&(sym_1485)), ((uint32_t)sym_1490));
                        }
                    } else {
                        sym_394((&(sym_1484)), ((uint32_t)sym_1487));
                        sym_394((&(sym_1485)), ((uint32_t)sym_1490));
                    }
                }
            }
            sym_1487 = (sym_1486 + 1);
        }
        sym_1486 = (sym_1486 + 1);
    }
    uint64_t sym_1497 = 0;
    uint64_t sym_1498 = 0;
    while ((sym_1498 < (sym_1484)._f567)) {
        sym_1497 = (sym_1497 + ((uint64_t)sym_396((&(sym_1485)), sym_1498)));
        if (((sym_1498 + 1) < (sym_1484)._f567)) {
            sym_1497 = (sym_1497 + 1);
        }
        sym_1498 = (sym_1498 + 1);
    }
    uint8_t* sym_1499 = sym_70(sym_1473, (sym_1497 + 1));
    uint64_t sym_1500 = 0;
    sym_1498 = 0;
    while ((sym_1498 < (sym_1484)._f567)) {
        uint64_t sym_1501 = ((uint64_t)sym_396((&(sym_1484)), sym_1498));
        uint64_t sym_1502 = ((uint64_t)sym_396((&(sym_1485)), sym_1498));
        uint64_t sym_1503 = 0;
        while ((sym_1503 < sym_1502)) {
            (*((sym_1499 + sym_1500))) = (*((((sym_1483)._f566 + sym_1501) + sym_1503)));
            sym_1500 = (sym_1500 + 1);
            sym_1503 = (sym_1503 + 1);
        }
        if (((sym_1498 + 1) < (sym_1484)._f567)) {
            (*((sym_1499 + sym_1500))) = ((uint8_t)47);
            sym_1500 = (sym_1500 + 1);
        }
        sym_1498 = (sym_1498 + 1);
    }
    (*((sym_1499 + sym_1500))) = ((uint8_t)0);
    sym_392((&(sym_1484)));
    sym_392((&(sym_1485)));
    return ((S_23){._f566 = sym_1499, ._f567 = sym_1497});
}

void sym_392(S_45* sym_1504) {
    sym_411(sym_1504);
    return;
}

void sym_411(S_45* sym_1505) {
    free(((uint8_t*)((*(sym_1505)))._f566));
    ((*(sym_1505)))._f566 = ((uint32_t*)0);
    ((*(sym_1505)))._f567 = 0;
    ((*(sym_1505)))._f554 = 0;
    return;
}

uint32_t sym_221(S_57* sym_1506, uint32_t sym_1507, uint32_t sym_1508) {
    uint32_t sym_1509 = sym_536((&(((*(sym_1506)))._f589)));
    uint32_t sym_1510 = 0;
    while ((sym_1510 < sym_1509)) {
        uint32_t sym_1511 = sym_538((&(((*(sym_1506)))._f590)), sym_1510);
        uint32_t sym_1512 = sym_538((&(((*(sym_1506)))._f589)), sym_1510);
        if (((sym_1511 == sym_1507) && (sym_1512 == sym_1508))) {
            uint32_t sym_1513 = sym_1510;
            while ((sym_1513 != sym_532())) {
                uint32_t sym_1514 = sym_538((&(((*(sym_1506)))._f595)), sym_1513);
                if ((sym_1514 == sym_532())) {
                    return sym_1513;
                }
                sym_1513 = sym_1514;
            }
            return sym_1510;
        }
        sym_1510 = (sym_1510 + 1);
    }
    return sym_532();
}

uint32_t sym_536(S_56* sym_1515) {
    return ((uint32_t)(((*(sym_1515)))._f621)._f567);
}

uint32_t sym_538(S_56* sym_1516, uint32_t sym_1517) {
    return sym_396((&(((*(sym_1516)))._f621)), ((uint64_t)sym_1517));
}

uint32_t sym_220(S_57* sym_1518, uint32_t sym_1519, uint32_t sym_1520, uint32_t sym_1521) {
    uint32_t sym_1522 = sym_537((&(((*(sym_1518)))._f589)), sym_1520);
    sym_537((&(((*(sym_1518)))._f590)), sym_1519);
    sym_537((&(((*(sym_1518)))._f591)), sym_532());
    sym_537((&(((*(sym_1518)))._f592)), sym_532());
    sym_537((&(((*(sym_1518)))._f593)), sym_532());
    sym_537((&(((*(sym_1518)))._f594)), sym_532());
    sym_537((&(((*(sym_1518)))._f596)), sym_532());
    sym_537((&(((*(sym_1518)))._f599)), 0);
    sym_537((&(((*(sym_1518)))._f595)), sym_1521);
    return sym_1522;
}

void sym_397(S_45* sym_1523, uint64_t sym_1524, uint32_t sym_1525) {
    sym_417(sym_1523, sym_1524, sym_1525);
    return;
}

void sym_417(S_45* sym_1526, uint64_t sym_1527, uint32_t sym_1528) {
    (*((((*(sym_1526)))._f566 + sym_1527))) = sym_1528;
    return;
}

uint32_t sym_314(S_59* sym_1529, uint32_t sym_1530, S_23 sym_1531) {
    uint32_t sym_1532 = sym_211((&((((*(sym_1529)))._f579)._f586)), sym_1531);
    return sym_221((&(((*(sym_1529)))._f579)), sym_1530, sym_1532);
}

void sym_315(S_59* sym_1533, uint32_t sym_1534) {
    uint32_t sym_1535 = sym_538((&((((*(sym_1533)))._f579)._f590)), sym_1534);
    uint32_t sym_1536 = sym_538((&((((*(sym_1533)))._f579)._f591)), sym_1534);
    uint32_t sym_1537 = sym_538((&((((*(sym_1533)))._f579)._f592)), sym_1534);
    uint32_t sym_1538 = (sym_1537 - sym_1536);
    uint32_t sym_1539 = sym_92((&((((*(sym_1533)))._f579)._f583)));
    S_333 sym_1540 = sym_103((&(((*(sym_1533)))._f579)), (&((((*(sym_1533)))._f579)._f582)), (&((((*(sym_1533)))._f579)._f583)), sym_1535, sym_1534, sym_1536, sym_1538);
    sym_117((&(sym_1540)), 0);
    sym_527((&((sym_1540)._f838)));
    uint32_t sym_1541 = sym_92((&((((*(sym_1533)))._f579)._f583)));
    sym_539((&((((*(sym_1533)))._f579)._f593)), sym_1534, sym_1539);
    sym_539((&((((*(sym_1533)))._f579)._f594)), sym_1534, sym_1541);
    return;
}

uint32_t sym_92(S_51* sym_1542) {
    return ((uint32_t)(((*(sym_1542)))._f606)._f567);
}

S_333 sym_103(S_57* sym_1543, S_50* sym_1544, S_51* sym_1545, uint32_t sym_1546, uint32_t sym_1547, uint32_t sym_1548, uint32_t sym_1549) {
    return ((S_333){._f748 = sym_1543, ._f582 = sym_1544, ._f583 = sym_1545, ._f664 = sym_1548, ._f841 = (sym_1548 + sym_1549), ._f658 = sym_1546, ._f840 = sym_1547, ._f838 = sym_526(), ._f842 = true});
}

S_332 sym_526(void) {
    return ((S_332){._f843 = sym_390(), ._f844 = sym_390()});
}

uint32_t sym_117(S_333* sym_1550, uint8_t sym_1551) {
    uint32_t sym_1552 = sym_118(sym_1550);
    while ((!(sym_105(sym_1550)))) {
        uint8_t sym_1553 = sym_106(sym_1550);
        if ((sym_111(sym_1550) && (!(sym_549(sym_1553))))) {
            break;
        }
        S_483 sym_1554 = sym_548(sym_1553);
        if (((!((sym_1554)._f847)) || ((sym_1554)._f848 < sym_1551))) {
            break;
        }
        sym_108(sym_1550);
        sym_1552 = sym_119(sym_1550, sym_1553, sym_1552, (sym_1554)._f849);
    }
    return sym_1552;
}

uint32_t sym_118(S_333* sym_1555) {
    S_222 sym_1556 = sym_8();
    uint8_t sym_1557 = sym_106(sym_1555);
    if ((sym_1557 == (sym_1556)._f667)) {
        return sym_85();
    }
    if ((sym_1557 == (sym_1556)._f670)) {
        return sym_125(sym_1555);
    }
    if ((sym_1557 == (sym_1556)._f672)) {
        return sym_136(sym_1555);
    }
    if ((sym_1557 == (sym_1556)._f674)) {
        return sym_137(sym_1555);
    }
    if ((sym_1557 == (sym_1556)._f685)) {
        return sym_138(sym_1555);
    }
    if ((sym_1557 == (sym_1556)._f686)) {
        return sym_139(sym_1555);
    }
    if ((sym_1557 == (sym_1556)._f673)) {
        return sym_126(sym_1555);
    }
    if ((sym_1557 == (sym_1556)._f725)) {
        return sym_127(sym_1555);
    }
    if ((sym_1557 == (sym_1556)._f727)) {
        return sym_128(sym_1555);
    }
    if ((sym_1557 == (sym_1556)._f677)) {
        return sym_120(sym_1555);
    }
    if ((sym_1557 == (sym_1556)._f679)) {
        return sym_121(sym_1555);
    }
    if ((sym_1557 == (sym_1556)._f680)) {
        return sym_140(sym_1555);
    }
    if ((sym_1557 == (sym_1556)._f681)) {
        return sym_141(sym_1555);
    }
    if ((sym_1557 == (sym_1556)._f676)) {
        return sym_130(sym_1555);
    }
    if ((sym_1557 == (sym_1556)._f682)) {
        return sym_142(sym_1555);
    }
    if (((sym_1557 == (sym_1556)._f723) && (sym_107(sym_1555, 1) == (sym_1556)._f727))) {
        return sym_143(sym_1555);
    }
    if ((((((sym_1557 == (sym_1556)._f692) || (sym_1557 == (sym_1556)._f690)) || (sym_1557 == (sym_1556)._f704)) || (sym_1557 == (sym_1556)._f701)) || (sym_1557 == (sym_1556)._f693))) {
        return sym_129(sym_1555, sym_1557);
    }
    sym_108(sym_1555);
    return sym_93(((*(sym_1555)))._f583, (sym_87())._f641, sym_85(), sym_85(), sym_85());
}

uint8_t sym_106(S_333* sym_1558) {
    if ((((*(sym_1558)))._f664 >= ((*(sym_1558)))._f841)) {
        return (sym_8())._f667;
    }
    return sym_15(((*(sym_1558)))._f582, ((*(sym_1558)))._f664);
}

uint32_t sym_125(S_333* sym_1559) {
    S_351 sym_1560 = sym_87();
    uint32_t sym_1561 = sym_114(sym_112(sym_1559));
    sym_108(sym_1559);
    return sym_93(((*(sym_1559)))._f583, (sym_1560)._f670, sym_1561, sym_85(), sym_85());
}

S_351 sym_87(void) {
    return ((S_351){._f773 = 0, ._f670 = 1, ._f672 = 2, ._f850 = 3, ._f851 = 4, ._f674 = 5, ._f675 = 6, ._f673 = 7, ._f852 = 8, ._f853 = 9, ._f854 = 10, ._f855 = 11, ._f856 = 12, ._f857 = 13, ._f858 = 14, ._f859 = 15, ._f860 = 16, ._f861 = 17, ._f862 = 18, ._f863 = 19, ._f864 = 20, ._f865 = 21, ._f866 = 22, ._f867 = 23, ._f868 = 24, ._f869 = 25, ._f870 = 26, ._f871 = 27, ._f872 = 28, ._f873 = 29, ._f874 = 30, ._f875 = 31, ._f876 = 32, ._f877 = 33, ._f878 = 34, ._f879 = 35, ._f880 = 36, ._f881 = 37, ._f882 = 38, ._f883 = 39, ._f884 = 40, ._f885 = 41, ._f886 = 42, ._f887 = 43, ._f888 = 44, ._f889 = 45, ._f890 = 46, ._f891 = 47, ._f892 = 48, ._f893 = 49, ._f894 = 50, ._f895 = 51, ._f896 = 52, ._f897 = 53, ._f898 = 54, ._f650 = 55, ._f899 = 56, ._f651 = 57, ._f900 = 58, ._f901 = 59, ._f902 = 60, ._f903 = 61, ._f641 = 62, ._f904 = 63});
}

uint32_t sym_114(S_23 sym_1562) {
    uint32_t sym_1563 = 0;
    uint64_t sym_1564 = 0;
    while ((sym_1564 < (sym_1562)._f567)) {
        uint8_t sym_1565 = (*(((sym_1562)._f566 + sym_1564)));
        if (((sym_1565 >= 48) && (sym_1565 <= 57))) {
            sym_1563 = ((sym_1563 * 10) + ((uint32_t)(sym_1565 - 48)));
        }
        sym_1564 = (sym_1564 + 1);
    }
    return sym_1563;
}

S_23 sym_112(S_333* sym_1566) {
    if ((((*(sym_1566)))._f664 >= ((*(sym_1566)))._f841)) {
        return sym_49();
    }
    return sym_18(((*(sym_1566)))._f582, ((*(sym_1566)))._f664);
}

uint32_t sym_108(S_333* sym_1567) {
    uint32_t sym_1568 = ((*(sym_1567)))._f664;
    if ((((*(sym_1567)))._f664 < ((*(sym_1567)))._f841)) {
        ((*(sym_1567)))._f664 = (((*(sym_1567)))._f664 + 1);
    }
    return sym_1568;
}

uint32_t sym_93(S_51* sym_1569, uint8_t sym_1570, uint32_t sym_1571, uint32_t sym_1572, uint32_t sym_1573) {
    uint32_t sym_1574 = ((uint32_t)(((*(sym_1569)))._f606)._f567);
    sym_352((&(((*(sym_1569)))._f606)), sym_1570);
    sym_394((&(((*(sym_1569)))._f607)), sym_1571);
    sym_394((&(((*(sym_1569)))._f608)), sym_1572);
    sym_394((&(((*(sym_1569)))._f609)), sym_1573);
    return sym_1574;
}

uint32_t sym_136(S_333* sym_1575) {
    S_351 sym_1576 = sym_87();
    S_23 sym_1577 = sym_112(sym_1575);
    uint32_t sym_1578 = sym_211((&(((*(((*(sym_1575)))._f748)))._f586)), sym_1577);
    sym_108(sym_1575);
    return sym_93(((*(sym_1575)))._f583, (sym_1576)._f672, sym_1578, sym_85(), sym_85());
}

uint32_t sym_137(S_333* sym_1579) {
    S_351 sym_1580 = sym_87();
    S_23 sym_1581 = sym_112(sym_1579);
    sym_108(sym_1579);
    uint32_t sym_1582 = sym_211((&(((*(((*(sym_1579)))._f748)))._f586)), sym_1581);
    return sym_93(((*(sym_1579)))._f583, (sym_1580)._f674, sym_1582, sym_85(), sym_85());
}

uint32_t sym_138(S_333* sym_1583) {
    S_351 sym_1584 = sym_87();
    sym_108(sym_1583);
    return sym_93(((*(sym_1583)))._f583, (sym_1584)._f850, sym_85(), sym_85(), sym_85());
}

uint32_t sym_139(S_333* sym_1585) {
    S_351 sym_1586 = sym_87();
    sym_108(sym_1585);
    return sym_93(((*(sym_1585)))._f583, (sym_1586)._f851, sym_85(), sym_85(), sym_85());
}

uint32_t sym_126(S_333* sym_1587) {
    S_351 sym_1588 = sym_87();
    S_222 sym_1589 = sym_8();
    if ((sym_106(sym_1587) != (sym_1589)._f673)) {
        sym_108(sym_1587);
        return sym_93(((*(sym_1587)))._f583, (sym_1588)._f641, sym_85(), sym_85(), sym_85());
    }
    S_23 sym_1590 = sym_112(sym_1587);
    uint32_t sym_1591 = sym_211((&(((*(((*(sym_1587)))._f748)))._f586)), sym_1590);
    sym_108(sym_1587);
    uint32_t sym_1592 = sym_93(((*(sym_1587)))._f583, (sym_1588)._f673, sym_1591, sym_85(), sym_85());
    uint32_t sym_1593 = sym_104(sym_1587, sym_1591);
    if ((sym_1593 != sym_532())) {
        sym_224(((*(sym_1587)))._f748, sym_1592, sym_1593);
    }
    return sym_1592;
}

uint32_t sym_104(S_333* sym_1594, uint32_t sym_1595) {
    uint32_t sym_1596 = sym_531((&(((*(sym_1594)))._f838)), sym_1595);
    if ((sym_1596 != sym_532())) {
        return sym_1596;
    }
    return sym_221(((*(sym_1594)))._f748, ((*(sym_1594)))._f658, sym_1595);
}

uint32_t sym_531(S_332* sym_1597, uint32_t sym_1598) {
    uint64_t sym_1599 = sym_405((&(((*(sym_1597)))._f843)));
    if ((sym_1599 == 0)) {
        return sym_532();
    }
    uint64_t sym_1600 = sym_1599;
    while ((sym_1600 > 0)) {
        sym_1600 = (sym_1600 - 1);
        uint32_t sym_1601 = sym_396((&(((*(sym_1597)))._f843)), sym_1600);
        if ((sym_1601 == sym_1598)) {
            return sym_396((&(((*(sym_1597)))._f844)), sym_1600);
        }
    }
    return sym_532();
}

uint64_t sym_405(S_45* sym_1602) {
    return sym_425(sym_1602);
}

uint64_t sym_425(S_45* sym_1603) {
    return ((*(sym_1603)))._f567;
}

void sym_224(S_57* sym_1604, uint32_t sym_1605, uint32_t sym_1606) {
    sym_541((&(((*(sym_1604)))._f597)), sym_1605, sym_1606, sym_532());
    return;
}

void sym_541(S_56* sym_1607, uint32_t sym_1608, uint32_t sym_1609, uint32_t sym_1610) {
    uint64_t sym_1611 = ((uint64_t)sym_1608);
    if ((sym_1611 >= (((*(sym_1607)))._f621)._f567)) {
        sym_540(sym_1607, ((uint32_t)(sym_1608 + 1)), sym_1610);
    }
    sym_397((&(((*(sym_1607)))._f621)), sym_1611, sym_1609);
    return;
}

void sym_540(S_56* sym_1612, uint32_t sym_1613, uint32_t sym_1614) {
    uint64_t sym_1615 = ((uint64_t)sym_1613);
    while (((((*(sym_1612)))._f621)._f567 < sym_1615)) {
        sym_394((&(((*(sym_1612)))._f621)), sym_1614);
    }
    return;
}

uint32_t sym_127(S_333* sym_1616) {
    S_222 sym_1617 = sym_8();
    S_351 sym_1618 = sym_87();
    sym_108(sym_1616);
    if ((((*(sym_1616)))._f842 && sym_109(sym_1616, (sym_1617)._f726))) {
        uint32_t sym_1619 = sym_528((&(((*(sym_1616)))._f838)));
        uint32_t sym_1620 = sym_124(sym_1616, sym_85());
        sym_530((&(((*(sym_1616)))._f838)), sym_1619);
        return sym_1620;
    }
    if ((((*(sym_1616)))._f842 && sym_123(sym_1616))) {
        uint32_t sym_1621 = sym_528((&(((*(sym_1616)))._f838)));
        S_45 sym_1622 = sym_390();
        S_45 sym_1623 = sym_390();
        while (((sym_106(sym_1616) != (sym_1617)._f726) && (!(sym_105(sym_1616))))) {
            uint32_t sym_1624 = sym_126(sym_1616);
            sym_122(sym_1616, sym_1624);
            sym_394((&(sym_1623)), sym_1624);
            if (sym_109(sym_1616, (sym_1617)._f722)) {
                continue;
            }
            if (((sym_106(sym_1616) != (sym_1617)._f726) && (sym_106(sym_1616) != (sym_1617)._f667))) {
                uint32_t sym_1625 = sym_117(sym_1616, ((uint8_t)0));
                uint64_t sym_1626 = 0;
                while ((sym_1626 < (sym_1623)._f567)) {
                    uint32_t sym_1627 = sym_396((&(sym_1623)), sym_1626);
                    uint32_t sym_1628 = sym_93(((*(sym_1616)))._f583, (sym_1618)._f887, sym_1627, sym_1625, sym_85());
                    sym_394((&(sym_1622)), sym_1628);
                    sym_1626 = (sym_1626 + 1);
                }
                sym_402((&(sym_1623)), 0);
                sym_109(sym_1616, (sym_1617)._f722);
            }
        }
        uint64_t sym_1629 = 0;
        while ((sym_1629 < (sym_1623)._f567)) {
            uint32_t sym_1630 = sym_396((&(sym_1623)), sym_1629);
            uint32_t sym_1631 = sym_93(((*(sym_1616)))._f583, (sym_1618)._f887, sym_1630, sym_85(), sym_85());
            sym_394((&(sym_1622)), sym_1631);
            sym_1629 = (sym_1629 + 1);
        }
        sym_110(sym_1616, (sym_1617)._f726);
        S_430 sym_1632 = ((S_430){._f566 = sym_404((&(sym_1622)), 0), ._f567 = (sym_1622)._f567});
        uint32_t sym_1633 = sym_216(((*(sym_1616)))._f748, sym_1632);
        sym_392((&(sym_1623)));
        sym_392((&(sym_1622)));
        uint32_t sym_1634 = sym_124(sym_1616, sym_1633);
        sym_530((&(((*(sym_1616)))._f838)), sym_1621);
        return sym_1634;
    }
    uint32_t sym_1635 = sym_117(sym_1616, ((uint8_t)0));
    sym_110(sym_1616, (sym_1617)._f726);
    return sym_1635;
}

bool sym_109(S_333* sym_1636, uint8_t sym_1637) {
    if ((sym_106(sym_1636) == sym_1637)) {
        sym_108(sym_1636);
        return true;
    }
    return false;
}

uint32_t sym_528(S_332* sym_1638) {
    return ((uint32_t)sym_405((&(((*(sym_1638)))._f843))));
}

uint32_t sym_124(S_333* sym_1639, uint32_t sym_1640) {
    ((*(sym_1639)))._f840 = sym_532();
    S_222 sym_1641 = sym_8();
    S_351 sym_1642 = sym_87();
    uint32_t sym_1643 = sym_85();
    if ((sym_106(sym_1639) != (sym_1641)._f727)) {
        sym_1643 = sym_117(sym_1639, ((uint8_t)0));
    }
    uint32_t sym_1644 = sym_85();
    if ((sym_106(sym_1639) == (sym_1641)._f727)) {
        sym_1644 = sym_128(sym_1639);
    }
    return sym_93(((*(sym_1639)))._f583, (sym_1642)._f888, sym_1640, sym_1643, sym_1644);
}

uint32_t sym_128(S_333* sym_1645) {
    S_351 sym_1646 = sym_87();
    S_222 sym_1647 = sym_8();
    S_57* sym_1648 = (&((*(((*(sym_1645)))._f748))));
    S_45* sym_1649 = (&(((*(sym_1648)))._f584));
    uint64_t sym_1650 = sym_401(sym_1649);
    uint32_t sym_1651 = sym_528((&(((*(sym_1645)))._f838)));
    sym_108(sym_1645);
    while (true) {
        uint8_t sym_1652 = sym_106(sym_1645);
        if ((sym_1652 == (sym_1647)._f728)) {
            sym_108(sym_1645);
            break;
        }
        if ((sym_1652 == (sym_1647)._f667)) {
            break;
        }
        sym_394(sym_1649, sym_117(sym_1645, 0));
    }
    S_430 sym_1653 = ((S_430){._f566 = sym_404(sym_1649, sym_1650), ._f567 = (sym_405(sym_1649) - sym_1650)});
    uint32_t sym_1654 = sym_216(sym_1648, sym_1653);
    sym_402(sym_1649, sym_1650);
    sym_530((&(((*(sym_1645)))._f838)), sym_1651);
    return sym_93(((*(sym_1645)))._f583, (sym_1646)._f893, sym_1654, sym_85(), sym_85());
}

uint64_t sym_401(S_45* sym_1655) {
    return sym_421(sym_1655);
}

uint64_t sym_421(S_45* sym_1656) {
    return ((*(sym_1656)))._f567;
}

uint32_t* sym_404(S_45* sym_1657, uint64_t sym_1658) {
    return sym_424(sym_1657, sym_1658);
}

uint32_t* sym_424(S_45* sym_1659, uint64_t sym_1660) {
    return (((*(sym_1659)))._f566 + sym_1660);
}

uint32_t sym_216(S_57* sym_1661, S_430 sym_1662) {
    S_45* sym_1663 = (&(((*(sym_1661)))._f585));
    uint32_t sym_1664 = ((uint32_t)sym_405(sym_1663));
    sym_394(sym_1663, ((uint32_t)(sym_1662)._f567));
    sym_400(sym_1663, (sym_1662)._f566, (sym_1662)._f567);
    return sym_1664;
}

void sym_400(S_45* sym_1665, uint32_t* sym_1666, uint64_t sym_1667) {
    sym_420(sym_1665, sym_1666, sym_1667);
    return;
}

void sym_420(S_45* sym_1668, uint32_t* sym_1669, uint64_t sym_1670) {
    if ((sym_1670 == 0)) {
        return;
    }
    sym_412(sym_1668, sym_1670);
    uint8_t* sym_1671 = ((uint8_t*)(((*(sym_1668)))._f566 + ((*(sym_1668)))._f567));
    mem_copy(sym_1671, ((uint8_t*)sym_1669), (sym_1670 * sym_389()));
    ((*(sym_1668)))._f567 = (((*(sym_1668)))._f567 + sym_1670);
    return;
}

void sym_402(S_45* sym_1675, uint64_t sym_1676) {
    sym_422(sym_1675, sym_1676);
    return;
}

void sym_422(S_45* sym_1677, uint64_t sym_1678) {
    if ((sym_1678 <= ((*(sym_1677)))._f567)) {
        ((*(sym_1677)))._f567 = sym_1678;
    }
    return;
}

void sym_530(S_332* sym_1679, uint32_t sym_1680) {
    sym_402((&(((*(sym_1679)))._f843)), ((uint64_t)sym_1680));
    sym_402((&(((*(sym_1679)))._f844)), ((uint64_t)sym_1680));
    return;
}

bool sym_123(S_333* sym_1681) {
    S_222 sym_1682 = sym_8();
    if ((sym_106(sym_1681) != (sym_1682)._f673)) {
        return false;
    }
    uint64_t sym_1683 = 1;
    uint32_t sym_1684 = ((*(sym_1681)))._f664;
    uint64_t sym_1685 = false;
    while ((sym_1684 < ((*(sym_1681)))._f841)) {
        uint8_t sym_1686 = sym_15(((*(sym_1681)))._f582, sym_1684);
        if ((sym_1686 == (sym_1682)._f725)) {
            sym_1683 = (sym_1683 + 1);
        } else {
            if ((sym_1686 == (sym_1682)._f726)) {
                sym_1683 = (sym_1683 - 1);
                if ((sym_1683 == 0)) {
                    break;
                }
            } else {
                if (((sym_1686 == (sym_1682)._f722) && (sym_1683 == 1))) {
                    sym_1685 = true;
                }
            }
        }
        sym_1684 = (sym_1684 + 1);
    }
    if ((sym_1683 != 0)) {
        return false;
    }
    if (sym_1685) {
        return true;
    }
    uint8_t sym_1687 = (sym_1682)._f667;
    if (((sym_1684 + 1) < ((*(sym_1681)))._f841)) {
        sym_1687 = sym_15(((*(sym_1681)))._f582, (sym_1684 + 1));
    }
    if ((sym_1687 == (sym_1682)._f727)) {
        return true;
    }
    if ((sym_1687 == (sym_1682)._f673)) {
        if ((!(sym_19(((*(sym_1681)))._f582, (sym_1684 + 1))))) {
            return true;
        }
    }
    if ((((sym_1687 == (sym_1682)._f701) || (sym_1687 == (sym_1682)._f693)) || (sym_1687 == (sym_1682)._f729))) {
        if ((((*(sym_1681)))._f840 != sym_532())) {
            return true;
        }
    }
    return false;
}

bool sym_19(S_50* sym_1688, uint32_t sym_1689) {
    return sym_480((&(((*(sym_1688)))._f603)), ((uint64_t)sym_1689));
}

bool sym_480(S_49* sym_1690, uint64_t sym_1691) {
    return sym_500(sym_1690, sym_1691);
}

bool sym_500(S_49* sym_1692, uint64_t sym_1693) {
    return (*((((*(sym_1692)))._f566 + sym_1693)));
}

bool sym_105(S_333* sym_1694) {
    return (((*(sym_1694)))._f664 >= ((*(sym_1694)))._f841);
}

void sym_122(S_333* sym_1695, uint32_t sym_1696) {
    S_351 sym_1697 = sym_87();
    if ((sym_94(((*(sym_1695)))._f583, sym_1696) == (sym_1697)._f673)) {
        uint32_t sym_1698 = sym_95(((*(sym_1695)))._f583, sym_1696);
        uint32_t sym_1699 = sym_219(((*(sym_1695)))._f748, ((*(sym_1695)))._f658, sym_1698, sym_1696);
        sym_529((&(((*(sym_1695)))._f838)), sym_1698, sym_1699);
    }
    return;
}

uint8_t sym_94(S_51* sym_1700, uint32_t sym_1701) {
    return sym_354((&(((*(sym_1700)))._f606)), ((uint64_t)sym_1701));
}

uint32_t sym_95(S_51* sym_1702, uint32_t sym_1703) {
    return sym_396((&(((*(sym_1702)))._f607)), ((uint64_t)sym_1703));
}

uint32_t sym_219(S_57* sym_1704, uint32_t sym_1705, uint32_t sym_1706, uint32_t sym_1707) {
    uint32_t sym_1708 = sym_537((&(((*(sym_1704)))._f589)), sym_1706);
    sym_537((&(((*(sym_1704)))._f590)), sym_1705);
    sym_537((&(((*(sym_1704)))._f591)), sym_532());
    sym_537((&(((*(sym_1704)))._f592)), sym_532());
    sym_537((&(((*(sym_1704)))._f593)), sym_532());
    sym_537((&(((*(sym_1704)))._f594)), sym_532());
    sym_537((&(((*(sym_1704)))._f596)), sym_1707);
    sym_537((&(((*(sym_1704)))._f599)), 0);
    sym_537((&(((*(sym_1704)))._f595)), sym_532());
    sym_224(sym_1704, sym_1707, sym_1708);
    return sym_1708;
}

void sym_529(S_332* sym_1709, uint32_t sym_1710, uint32_t sym_1711) {
    sym_394((&(((*(sym_1709)))._f843)), sym_1710);
    sym_394((&(((*(sym_1709)))._f844)), sym_1711);
    return;
}

bool sym_110(S_333* sym_1712, uint8_t sym_1713) {
    if ((sym_106(sym_1712) == sym_1713)) {
        sym_108(sym_1712);
        return true;
    }
    return false;
}

uint32_t sym_120(S_333* sym_1714) {
    S_222 sym_1715 = sym_8();
    S_351 sym_1716 = sym_87();
    sym_108(sym_1714);
    bool sym_1717 = ((*(sym_1714)))._f842;
    ((*(sym_1714)))._f842 = false;
    uint32_t sym_1718 = sym_117(sym_1714, ((uint8_t)0));
    ((*(sym_1714)))._f842 = sym_1717;
    uint32_t sym_1719 = sym_117(sym_1714, ((uint8_t)0));
    uint32_t sym_1720 = sym_85();
    if (sym_109(sym_1714, (sym_1715)._f678)) {
        sym_1720 = sym_117(sym_1714, ((uint8_t)0));
    }
    return sym_93(((*(sym_1714)))._f583, (sym_1716)._f894, sym_1718, sym_1719, sym_1720);
}

uint32_t sym_121(S_333* sym_1721) {
    S_351 sym_1722 = sym_87();
    sym_108(sym_1721);
    bool sym_1723 = ((*(sym_1721)))._f842;
    ((*(sym_1721)))._f842 = false;
    uint32_t sym_1724 = sym_117(sym_1721, ((uint8_t)0));
    ((*(sym_1721)))._f842 = sym_1723;
    uint32_t sym_1725 = sym_117(sym_1721, ((uint8_t)0));
    return sym_93(((*(sym_1721)))._f583, (sym_1722)._f895, sym_1724, sym_1725, sym_85());
}

uint32_t sym_140(S_333* sym_1726) {
    S_351 sym_1727 = sym_87();
    sym_108(sym_1726);
    return sym_93(((*(sym_1726)))._f583, (sym_1727)._f896, sym_85(), sym_85(), sym_85());
}

uint32_t sym_141(S_333* sym_1728) {
    S_351 sym_1729 = sym_87();
    sym_108(sym_1728);
    return sym_93(((*(sym_1728)))._f583, (sym_1729)._f897, sym_85(), sym_85(), sym_85());
}

uint32_t sym_130(S_333* sym_1730) {
    S_222 sym_1731 = sym_8();
    S_351 sym_1732 = sym_87();
    sym_108(sym_1730);
    uint32_t sym_1733 = sym_85();
    if ((((!(sym_111(sym_1730))) && (sym_106(sym_1730) != (sym_1731)._f728)) && (sym_106(sym_1730) != (sym_1731)._f667))) {
        sym_1733 = sym_117(sym_1730, ((uint8_t)0));
    }
    return sym_93(((*(sym_1730)))._f583, (sym_1732)._f892, sym_1733, sym_85(), sym_85());
}

bool sym_111(S_333* sym_1734) {
    if ((((*(sym_1734)))._f664 >= ((*(sym_1734)))._f841)) {
        return false;
    }
    return sym_19(((*(sym_1734)))._f582, ((*(sym_1734)))._f664);
}

uint32_t sym_142(S_333* sym_1735) {
    S_222 sym_1736 = sym_8();
    S_351 sym_1737 = sym_87();
    sym_108(sym_1735);
    sym_110(sym_1735, (sym_1736)._f727);
    S_45 sym_1738 = sym_390();
    S_45 sym_1739 = sym_390();
    while (true) {
        uint8_t sym_1740 = sym_106(sym_1735);
        if (((sym_1740 == (sym_1736)._f728) || (sym_1740 == (sym_1736)._f667))) {
            break;
        }
        if ((sym_106(sym_1735) == (sym_1736)._f722)) {
            sym_108(sym_1735);
        }
        if (((sym_106(sym_1735) == (sym_1736)._f728) || (sym_106(sym_1735) == (sym_1736)._f667))) {
            break;
        }
        uint32_t sym_1741 = sym_126(sym_1735);
        sym_110(sym_1735, (sym_1736)._f719);
        uint32_t sym_1742 = sym_117(sym_1735, ((sym_543())._f939 + ((uint8_t)1)));
        sym_394((&(sym_1738)), sym_1741);
        sym_394((&(sym_1739)), sym_1742);
        uint8_t sym_1743 = sym_106(sym_1735);
        if (((sym_1743 == (sym_1736)._f728) || (sym_1743 == (sym_1736)._f667))) {
            break;
        }
        if ((sym_106(sym_1735) == (sym_1736)._f722)) {
            sym_108(sym_1735);
        }
    }
    sym_110(sym_1735, (sym_1736)._f728);
    uint32_t* sym_1744 = ((uint32_t*)0);
    if (((sym_1738)._f567 > 0)) {
        sym_1744 = sym_404((&(sym_1738)), 0);
    }
    S_430 sym_1745 = ((S_430){._f566 = sym_1744, ._f567 = (sym_1738)._f567});
    uint32_t* sym_1746 = ((uint32_t*)0);
    if (((sym_1739)._f567 > 0)) {
        sym_1746 = sym_404((&(sym_1739)), 0);
    }
    S_430 sym_1747 = ((S_430){._f566 = sym_1746, ._f567 = (sym_1739)._f567});
    uint32_t sym_1748 = sym_216(((*(sym_1735)))._f748, sym_1745);
    uint32_t sym_1749 = sym_216(((*(sym_1735)))._f748, sym_1747);
    sym_392((&(sym_1738)));
    sym_392((&(sym_1739)));
    return sym_93(((*(sym_1735)))._f583, (sym_1737)._f650, sym_1748, sym_1749, sym_85());
}

S_449 sym_543(void) {
    return ((S_449){._f947 = 0, ._f948 = 5, ._f949 = 10, ._f939 = 15, ._f950 = 30, ._f951 = 40, ._f952 = 60, ._f953 = 70, ._f954 = 80, ._f955 = 90, ._f956 = 100, ._f957 = 110, ._f958 = 120, ._f898 = 130, ._f959 = 140, ._f889 = 150, ._f902 = 160});
}

uint8_t sym_107(S_333* sym_1750, uint32_t sym_1751) {
    uint32_t sym_1752 = (((*(sym_1750)))._f664 + sym_1751);
    if ((sym_1752 >= ((*(sym_1750)))._f841)) {
        return (sym_8())._f667;
    }
    return sym_15(((*(sym_1750)))._f582, sym_1752);
}

uint32_t sym_143(S_333* sym_1753) {
    S_222 sym_1754 = sym_8();
    S_351 sym_1755 = sym_87();
    sym_108(sym_1753);
    sym_110(sym_1753, (sym_1754)._f727);
    S_45 sym_1756 = sym_390();
    S_45 sym_1757 = sym_390();
    while (true) {
        uint8_t sym_1758 = sym_106(sym_1753);
        if (((sym_1758 == (sym_1754)._f728) || (sym_1758 == (sym_1754)._f667))) {
            break;
        }
        if ((sym_106(sym_1753) == (sym_1754)._f722)) {
            sym_108(sym_1753);
        }
        if (((sym_106(sym_1753) == (sym_1754)._f728) || (sym_106(sym_1753) == (sym_1754)._f667))) {
            break;
        }
        uint32_t sym_1759 = sym_126(sym_1753);
        sym_110(sym_1753, (sym_1754)._f719);
        uint32_t sym_1760 = sym_117(sym_1753, ((sym_543())._f939 + ((uint8_t)1)));
        sym_394((&(sym_1756)), sym_1759);
        sym_394((&(sym_1757)), sym_1760);
        uint8_t sym_1761 = sym_106(sym_1753);
        if (((sym_1761 == (sym_1754)._f728) || (sym_1761 == (sym_1754)._f667))) {
            break;
        }
        if ((sym_106(sym_1753) == (sym_1754)._f722)) {
            sym_108(sym_1753);
        }
    }
    sym_110(sym_1753, (sym_1754)._f728);
    uint32_t* sym_1762 = ((uint32_t*)0);
    if (((sym_1756)._f567 > 0)) {
        sym_1762 = sym_404((&(sym_1756)), 0);
    }
    S_430 sym_1763 = ((S_430){._f566 = sym_1762, ._f567 = (sym_1756)._f567});
    uint32_t* sym_1764 = ((uint32_t*)0);
    if (((sym_1757)._f567 > 0)) {
        sym_1764 = sym_404((&(sym_1757)), 0);
    }
    S_430 sym_1765 = ((S_430){._f566 = sym_1764, ._f567 = (sym_1757)._f567});
    uint32_t sym_1766 = sym_216(((*(sym_1753)))._f748, sym_1763);
    uint32_t sym_1767 = sym_216(((*(sym_1753)))._f748, sym_1765);
    sym_392((&(sym_1756)));
    sym_392((&(sym_1757)));
    return sym_93(((*(sym_1753)))._f583, (sym_1755)._f899, sym_1766, sym_1767, sym_85());
}

uint32_t sym_129(S_333* sym_1768, uint8_t sym_1769) {
    S_222 sym_1770 = sym_8();
    S_351 sym_1771 = sym_87();
    sym_108(sym_1768);
    uint32_t sym_1772 = sym_117(sym_1768, (sym_543())._f959);
    uint8_t sym_1773 = (sym_1771)._f773;
    if ((sym_1769 == (sym_1770)._f692)) {
        sym_1773 = (sym_1771)._f882;
    }
    if ((sym_1769 == (sym_1770)._f690)) {
        sym_1773 = (sym_1771)._f883;
    }
    if ((sym_1769 == (sym_1770)._f704)) {
        sym_1773 = (sym_1771)._f884;
    }
    if ((sym_1769 == (sym_1770)._f701)) {
        sym_1773 = (sym_1771)._f886;
    }
    if ((sym_1769 == (sym_1770)._f693)) {
        sym_1773 = (sym_1771)._f885;
    }
    return sym_93(((*(sym_1768)))._f583, sym_1773, sym_1772, sym_85(), sym_85());
}

bool sym_549(uint8_t sym_1774) {
    S_222 sym_1775 = sym_8();
    if (((((sym_1774 == (sym_1775)._f691) || (sym_1774 == (sym_1775)._f692)) || (sym_1774 == (sym_1775)._f694)) || (sym_1774 == (sym_1775)._f695))) {
        return true;
    }
    if (((sym_1774 == (sym_1775)._f719) || (sym_1774 == (sym_1775)._f720))) {
        return true;
    }
    if (((sym_1774 == (sym_1775)._f713) || (sym_1774 == (sym_1775)._f714))) {
        return true;
    }
    if (((((sym_1774 == (sym_1775)._f715) || (sym_1774 == (sym_1775)._f717)) || (sym_1774 == (sym_1775)._f716)) || (sym_1774 == (sym_1775)._f718))) {
        return true;
    }
    if (((sym_1774 == (sym_1775)._f688) || (sym_1774 == (sym_1775)._f689))) {
        return true;
    }
    if (((((sym_1774 == (sym_1775)._f702) || (sym_1774 == (sym_1775)._f703)) || (sym_1774 == (sym_1775)._f708)) || (sym_1774 == (sym_1775)._f709))) {
        return true;
    }
    if (((((((sym_1774 == (sym_1775)._f712) || (sym_1774 == (sym_1775)._f696)) || (sym_1774 == (sym_1775)._f697)) || (sym_1774 == (sym_1775)._f698)) || (sym_1774 == (sym_1775)._f699)) || (sym_1774 == (sym_1775)._f700))) {
        return true;
    }
    if ((((((sym_1774 == (sym_1775)._f705) || (sym_1774 == (sym_1775)._f706)) || (sym_1774 == (sym_1775)._f707)) || (sym_1774 == (sym_1775)._f710)) || (sym_1774 == (sym_1775)._f711))) {
        return true;
    }
    if ((sym_1774 == (sym_1775)._f687)) {
        return true;
    }
    return false;
}

S_483 sym_548(uint8_t sym_1776) {
    S_222 sym_1777 = sym_8();
    S_449 sym_1778 = sym_543();
    if ((sym_1776 == (sym_1777)._f719)) {
        return sym_546((sym_1778)._f939);
    }
    if ((sym_1776 == (sym_1777)._f720)) {
        return sym_546((sym_1778)._f948);
    }
    if ((sym_1776 == (sym_1777)._f712)) {
        return sym_546((sym_1778)._f949);
    }
    if ((((((sym_1776 == (sym_1777)._f696) || (sym_1776 == (sym_1777)._f697)) || (sym_1776 == (sym_1777)._f698)) || (sym_1776 == (sym_1777)._f699)) || (sym_1776 == (sym_1777)._f700))) {
        return sym_546((sym_1778)._f949);
    }
    if ((((((sym_1776 == (sym_1777)._f705) || (sym_1776 == (sym_1777)._f706)) || (sym_1776 == (sym_1777)._f707)) || (sym_1776 == (sym_1777)._f710)) || (sym_1776 == (sym_1777)._f711))) {
        return sym_546((sym_1778)._f949);
    }
    if (((sym_1776 == (sym_1777)._f691) || (sym_1776 == (sym_1777)._f692))) {
        return sym_545((sym_1778)._f957);
    }
    if ((((sym_1776 == (sym_1777)._f693) || (sym_1776 == (sym_1777)._f694)) || (sym_1776 == (sym_1777)._f695))) {
        return sym_545((sym_1778)._f958);
    }
    if ((sym_1776 == (sym_1777)._f701)) {
        return sym_545((sym_1778)._f955);
    }
    if ((sym_1776 == (sym_1777)._f702)) {
        return sym_545((sym_1778)._f953);
    }
    if ((sym_1776 == (sym_1777)._f703)) {
        return sym_545((sym_1778)._f954);
    }
    if (((sym_1776 == (sym_1777)._f708) || (sym_1776 == (sym_1777)._f709))) {
        return sym_545((sym_1778)._f956);
    }
    if ((sym_1776 == (sym_1777)._f688)) {
        return sym_545((sym_1778)._f951);
    }
    if ((sym_1776 == (sym_1777)._f689)) {
        return sym_545((sym_1778)._f950);
    }
    if (((((((sym_1776 == (sym_1777)._f717) || (sym_1776 == (sym_1777)._f718)) || (sym_1776 == (sym_1777)._f715)) || (sym_1776 == (sym_1777)._f716)) || (sym_1776 == (sym_1777)._f713)) || (sym_1776 == (sym_1777)._f714))) {
        return sym_545((sym_1778)._f952);
    }
    if (((sym_1776 == (sym_1777)._f725) || (sym_1776 == (sym_1777)._f729))) {
        return sym_545((sym_1778)._f889);
    }
    if ((sym_1776 == (sym_1777)._f687)) {
        return sym_545((sym_1778)._f898);
    }
    if ((sym_1776 == (sym_1777)._f723)) {
        return sym_545((sym_1778)._f902);
    }
    return sym_547();
}

S_483 sym_546(uint8_t sym_1779) {
    uint8_t sym_1780 = sym_1779;
    if ((sym_1779 > 0)) {
        sym_1780 = (sym_1779 - 1);
    }
    return ((S_483){._f848 = sym_1779, ._f849 = sym_1780, ._f847 = true});
}

S_483 sym_545(uint8_t sym_1781) {
    return ((S_483){._f848 = sym_1781, ._f849 = (sym_1781 + 1), ._f847 = true});
}

S_483 sym_547(void) {
    return ((S_483){._f848 = 0, ._f849 = 0, ._f847 = false});
}

uint32_t sym_119(S_333* sym_1782, uint8_t sym_1783, uint32_t sym_1784, uint8_t sym_1785) {
    S_222 sym_1786 = sym_8();
    if ((sym_1783 == (sym_1786)._f720)) {
        return sym_132(sym_1782, sym_1784, sym_1785);
    }
    if ((sym_1783 == (sym_1786)._f719)) {
        return sym_133(sym_1782, sym_1784, sym_1785);
    }
    if ((sym_1783 == (sym_1786)._f712)) {
        return sym_134(sym_1782, sym_1784, sym_1785);
    }
    if ((sym_1783 == (sym_1786)._f725)) {
        return sym_135(sym_1782, sym_1784, sym_1785);
    }
    if ((sym_1783 == (sym_1786)._f723)) {
        return sym_144(sym_1782, sym_1784, sym_1785);
    }
    return sym_131(sym_1782, sym_1783, sym_1784, sym_1785);
}

uint32_t sym_132(S_333* sym_1787, uint32_t sym_1788, uint8_t sym_1789) {
    S_351 sym_1790 = sym_87();
    uint32_t sym_1791 = sym_95(((*(sym_1787)))._f583, sym_1788);
    uint32_t sym_1792 = sym_104(sym_1787, sym_1791);
    if ((sym_1792 != sym_532())) {
        uint32_t sym_1793 = sym_223(((*(sym_1787)))._f748, sym_1792);
        if ((sym_1793 == sym_532())) {
            sym_222(((*(sym_1787)))._f748, sym_1792, sym_1788);
        }
        sym_224(((*(sym_1787)))._f748, sym_1788, sym_1792);
    }
    uint32_t sym_1794 = sym_117(sym_1787, sym_1789);
    uint32_t sym_1795 = sym_93(((*(sym_1787)))._f583, (sym_1790)._f890, sym_1788, sym_85(), sym_1794);
    if ((sym_1792 != sym_532())) {
        sym_224(((*(sym_1787)))._f748, sym_1795, sym_1792);
    }
    return sym_1795;
}

uint32_t sym_223(S_57* sym_1796, uint32_t sym_1797) {
    if ((sym_1797 == sym_532())) {
        return sym_532();
    }
    if ((sym_1797 >= sym_536((&(((*(sym_1796)))._f596))))) {
        return sym_532();
    }
    return sym_538((&(((*(sym_1796)))._f596)), sym_1797);
}

void sym_222(S_57* sym_1798, uint32_t sym_1799, uint32_t sym_1800) {
    sym_539((&(((*(sym_1798)))._f596)), sym_1799, sym_1800);
    return;
}

void sym_539(S_56* sym_1801, uint32_t sym_1802, uint32_t sym_1803) {
    sym_397((&(((*(sym_1801)))._f621)), ((uint64_t)sym_1802), sym_1803);
    return;
}

uint32_t sym_133(S_333* sym_1804, uint32_t sym_1805, uint8_t sym_1806) {
    S_222 sym_1807 = sym_8();
    S_351 sym_1808 = sym_87();
    uint32_t sym_1809 = sym_95(((*(sym_1804)))._f583, sym_1805);
    uint32_t sym_1810 = sym_219(((*(sym_1804)))._f748, ((*(sym_1804)))._f658, sym_1809, sym_1805);
    sym_529((&(((*(sym_1804)))._f838)), sym_1809, sym_1810);
    if (sym_109(sym_1804, (sym_1807)._f712)) {
        uint32_t sym_1811 = sym_117(sym_1804, ((uint8_t)0));
        uint32_t sym_1812 = sym_93(((*(sym_1804)))._f583, (sym_1808)._f891, sym_1805, sym_85(), sym_1811);
        sym_224(((*(sym_1804)))._f748, sym_1812, sym_1810);
        return sym_1812;
    }
    uint32_t sym_1813 = sym_117(sym_1804, ((sym_543())._f939 + ((uint8_t)1)));
    if (sym_109(sym_1804, (sym_1807)._f712)) {
        uint32_t sym_1814 = sym_117(sym_1804, ((uint8_t)0));
        uint32_t sym_1815 = sym_93(((*(sym_1804)))._f583, (sym_1808)._f891, sym_1805, sym_1813, sym_1814);
        sym_224(((*(sym_1804)))._f748, sym_1815, sym_1810);
        return sym_1815;
    }
    if (sym_109(sym_1804, (sym_1807)._f719)) {
        uint32_t sym_1816 = sym_117(sym_1804, ((uint8_t)0));
        uint32_t sym_1817 = sym_93(((*(sym_1804)))._f583, (sym_1808)._f890, sym_1805, sym_1813, sym_1816);
        sym_224(((*(sym_1804)))._f748, sym_1817, sym_1810);
        return sym_1817;
    }
    uint32_t sym_1818 = sym_93(((*(sym_1804)))._f583, (sym_1808)._f891, sym_1805, sym_1813, sym_85());
    sym_224(((*(sym_1804)))._f748, sym_1818, sym_1810);
    return sym_1818;
}

uint32_t sym_134(S_333* sym_1819, uint32_t sym_1820, uint8_t sym_1821) {
    S_351 sym_1822 = sym_87();
    uint32_t sym_1823 = sym_117(sym_1819, sym_1821);
    return sym_93(((*(sym_1819)))._f583, (sym_1822)._f852, sym_1820, sym_1823, sym_85());
}

uint32_t sym_135(S_333* sym_1824, uint32_t sym_1825, uint8_t sym_1826) {
    S_222 sym_1827 = sym_8();
    S_351 sym_1828 = sym_87();
    S_45 sym_1829 = sym_390();
    if ((!(sym_109(sym_1824, (sym_1827)._f726)))) {
        while (true) {
            uint32_t sym_1830 = sym_117(sym_1824, ((uint8_t)0));
            sym_394((&(sym_1829)), sym_1830);
            if ((!(sym_109(sym_1824, (sym_1827)._f722)))) {
                break;
            }
        }
        sym_110(sym_1824, (sym_1827)._f726);
    }
    S_430 sym_1831 = ((S_430){._f566 = sym_404((&(sym_1829)), 0), ._f567 = (sym_1829)._f567});
    uint32_t sym_1832 = sym_216(((*(sym_1824)))._f748, sym_1831);
    sym_392((&(sym_1829)));
    return sym_93(((*(sym_1824)))._f583, (sym_1828)._f889, sym_1825, sym_1832, sym_85());
}

uint32_t sym_144(S_333* sym_1833, uint32_t sym_1834, uint8_t sym_1835) {
    S_351 sym_1836 = sym_87();
    uint32_t sym_1837 = sym_117(sym_1833, sym_1835);
    return sym_93(((*(sym_1833)))._f583, (sym_1836)._f902, sym_1834, sym_1837, sym_85());
}

uint32_t sym_131(S_333* sym_1838, uint8_t sym_1839, uint32_t sym_1840, uint8_t sym_1841) {
    S_222 sym_1842 = sym_8();
    S_351 sym_1843 = sym_87();
    uint32_t sym_1844 = sym_117(sym_1838, sym_1841);
    uint8_t sym_1845 = (sym_1843)._f773;
    if ((sym_1839 == (sym_1842)._f691)) {
        sym_1845 = (sym_1843)._f863;
    }
    if ((sym_1839 == (sym_1842)._f692)) {
        sym_1845 = (sym_1843)._f864;
    }
    if ((sym_1839 == (sym_1842)._f693)) {
        sym_1845 = (sym_1843)._f865;
    }
    if ((sym_1839 == (sym_1842)._f694)) {
        sym_1845 = (sym_1843)._f866;
    }
    if ((sym_1839 == (sym_1842)._f695)) {
        sym_1845 = (sym_1843)._f867;
    }
    if ((sym_1839 == (sym_1842)._f701)) {
        sym_1845 = (sym_1843)._f876;
    }
    if ((sym_1839 == (sym_1842)._f702)) {
        sym_1845 = (sym_1843)._f877;
    }
    if ((sym_1839 == (sym_1842)._f703)) {
        sym_1845 = (sym_1843)._f878;
    }
    if ((sym_1839 == (sym_1842)._f708)) {
        sym_1845 = (sym_1843)._f879;
    }
    if ((sym_1839 == (sym_1842)._f709)) {
        sym_1845 = (sym_1843)._f880;
    }
    if ((sym_1839 == (sym_1842)._f713)) {
        sym_1845 = (sym_1843)._f868;
    }
    if ((sym_1839 == (sym_1842)._f714)) {
        sym_1845 = (sym_1843)._f869;
    }
    if ((sym_1839 == (sym_1842)._f715)) {
        sym_1845 = (sym_1843)._f870;
    }
    if ((sym_1839 == (sym_1842)._f716)) {
        sym_1845 = (sym_1843)._f872;
    }
    if ((sym_1839 == (sym_1842)._f717)) {
        sym_1845 = (sym_1843)._f871;
    }
    if ((sym_1839 == (sym_1842)._f718)) {
        sym_1845 = (sym_1843)._f873;
    }
    if ((sym_1839 == (sym_1842)._f688)) {
        sym_1845 = (sym_1843)._f874;
    }
    if ((sym_1839 == (sym_1842)._f689)) {
        sym_1845 = (sym_1843)._f875;
    }
    if ((sym_1839 == (sym_1842)._f687)) {
        sym_1845 = (sym_1843)._f898;
    }
    if ((sym_1845 == (sym_1843)._f773)) {
        return sym_93(((*(sym_1838)))._f583, (sym_1843)._f641, sym_85(), sym_85(), sym_85());
    }
    return sym_93(((*(sym_1838)))._f583, sym_1845, sym_1840, sym_1844, sym_85());
}

void sym_527(S_332* sym_1846) {
    sym_392((&(((*(sym_1846)))._f843)));
    sym_392((&(((*(sym_1846)))._f844)));
    return;
}

void sym_316(S_59* sym_1847, uint32_t sym_1848) {
    uint32_t sym_1849 = sym_538((&((((*(sym_1847)))._f579)._f593)), sym_1848);
    uint32_t sym_1850 = sym_538((&((((*(sym_1847)))._f579)._f594)), sym_1848);
    sym_159(sym_1847, sym_1849, sym_1850);
    return;
}

void sym_159(S_59* sym_1851, uint32_t sym_1852, uint32_t sym_1853) {
    S_57* sym_1854 = (&(((*(sym_1851)))._f579));
    S_51* sym_1855 = (&(((*(sym_1854)))._f583));
    uint32_t sym_1856 = sym_300((&(((*(sym_1854)))._f588)));
    uint32_t sym_1857 = sym_1852;
    while ((sym_1857 < sym_1853)) {
        sym_160(sym_1851, sym_1857, sym_1856);
        sym_1857 = (sym_1857 + 1);
    }
    return;
}

uint32_t sym_300(S_55* sym_1858) {
    uint32_t sym_1859 = ((uint32_t)(((*(sym_1858)))._f616)._f567);
    sym_394((&(((*(sym_1858)))._f616)), sym_1859);
    sym_352((&(((*(sym_1858)))._f617)), ((uint8_t)0));
    sym_394((&(((*(sym_1858)))._f587)), sym_236());
    sym_394((&(((*(sym_1858)))._f618)), sym_294());
    return sym_1859;
}

uint32_t sym_294(void) {
    return sym_524();
}

void sym_160(S_59* sym_1860, uint32_t sym_1861, uint32_t sym_1862) {
    S_57* sym_1863 = (&(((*(sym_1860)))._f579));
    S_51* sym_1864 = (&(((*(sym_1863)))._f583));
    S_351 sym_1865 = sym_87();
    uint8_t sym_1866 = sym_94(sym_1864, sym_1861);
    if ((sym_1866 == (sym_1865)._f670)) {
        sym_161(sym_1863, sym_1864, sym_1861);
    }
    if ((sym_1866 == (sym_1865)._f673)) {
        sym_162(sym_1860, sym_1861);
    }
    if ((sym_1866 == (sym_1865)._f891)) {
        sym_163(sym_1863, sym_1864, sym_1861);
    }
    if ((sym_1866 == (sym_1865)._f890)) {
        sym_163(sym_1863, sym_1864, sym_1861);
    }
    if ((sym_1866 == (sym_1865)._f887)) {
        sym_164(sym_1863, sym_1864, sym_1861);
    }
    if ((sym_1866 == (sym_1865)._f888)) {
        sym_165(sym_1863, sym_1864, sym_1861, sym_1862);
    }
    if ((sym_1866 == (sym_1865)._f892)) {
        sym_166(sym_1863, sym_1864, sym_1861, sym_1862);
    }
    if ((sym_1866 == (sym_1865)._f889)) {
        sym_167(sym_1863, sym_1864, sym_1861);
    }
    if ((sym_1866 == (sym_1865)._f893)) {
        sym_168(sym_1863, sym_1864, sym_1861);
    }
    if ((sym_1866 == (sym_1865)._f863)) {
        sym_169(sym_1863, sym_1864, sym_1861);
    }
    if ((sym_1866 == (sym_1865)._f852)) {
        sym_170(sym_1863, sym_1864, sym_1861);
    }
    if ((sym_1866 == (sym_1865)._f672)) {
        sym_171(sym_1863, sym_1864, sym_1861);
    }
    if (((sym_1866 == (sym_1865)._f850) || (sym_1866 == (sym_1865)._f851))) {
        sym_172(sym_1863, sym_1864, sym_1861);
    }
    if ((sym_1866 == (sym_1865)._f674)) {
        sym_173(sym_1863, sym_1864, sym_1861);
    }
    if (((sym_1866 == (sym_1865)._f882) || (sym_1866 == (sym_1865)._f884))) {
        sym_174(sym_1863, sym_1864, sym_1861);
    }
    if ((sym_1866 == (sym_1865)._f883)) {
        sym_175(sym_1863, sym_1864, sym_1861);
    }
    if ((sym_1866 == (sym_1865)._f886)) {
        sym_176(sym_1863, sym_1864, sym_1861);
    }
    if ((sym_1866 == (sym_1865)._f885)) {
        sym_177(sym_1863, sym_1864, sym_1861);
    }
    if (((sym_1866 == (sym_1865)._f896) || (sym_1866 == (sym_1865)._f897))) {
        sym_178(sym_1863, sym_1864, sym_1861);
    }
    if ((sym_1866 == (sym_1865)._f894)) {
        sym_179(sym_1863, sym_1864, sym_1861);
    }
    if ((sym_1866 == (sym_1865)._f895)) {
        sym_180(sym_1863, sym_1864, sym_1861);
    }
    if ((sym_1866 == (sym_1865)._f898)) {
        sym_183(sym_1863, sym_1864, sym_1861);
    }
    if ((sym_1866 == (sym_1865)._f650)) {
        sym_184(sym_1863, sym_1864, sym_1861);
    }
    if ((sym_1866 == (sym_1865)._f899)) {
        sym_185(sym_1863, sym_1864, sym_1861);
    }
    if ((sym_1866 == (sym_1865)._f902)) {
        sym_186(sym_1863, sym_1864, sym_1861);
    }
    if (((sym_1866 == (sym_1865)._f874) || (sym_1866 == (sym_1865)._f875))) {
        sym_182(sym_1863, sym_1864, sym_1861);
    }
    if (((((((((((sym_1866 == (sym_1865)._f863) || (sym_1866 == (sym_1865)._f864)) || (sym_1866 == (sym_1865)._f865)) || (sym_1866 == (sym_1865)._f866)) || (sym_1866 == (sym_1865)._f867)) || (sym_1866 == (sym_1865)._f876)) || (sym_1866 == (sym_1865)._f877)) || (sym_1866 == (sym_1865)._f878)) || (sym_1866 == (sym_1865)._f879)) || (sym_1866 == (sym_1865)._f880))) {
        sym_169(sym_1863, sym_1864, sym_1861);
    }
    if (((((((sym_1866 == (sym_1865)._f868) || (sym_1866 == (sym_1865)._f869)) || (sym_1866 == (sym_1865)._f870)) || (sym_1866 == (sym_1865)._f872)) || (sym_1866 == (sym_1865)._f871)) || (sym_1866 == (sym_1865)._f873))) {
        sym_181(sym_1863, sym_1864, sym_1861);
    }
    return;
}

void sym_161(S_57* sym_1867, S_51* sym_1868, uint32_t sym_1869) {
    S_55* sym_1870 = (&(((*(sym_1867)))._f588));
    S_54* sym_1871 = (&(((*(sym_1867)))._f587));
    uint32_t sym_1872 = sym_299(sym_1870, sym_1869);
    sym_304(sym_1870, sym_1871, sym_1872, sym_238());
    return;
}

uint32_t sym_299(S_55* sym_1873, uint32_t sym_1874) {
    uint32_t sym_1875 = ((uint32_t)(((*(sym_1873)))._f616)._f567);
    sym_394((&(((*(sym_1873)))._f616)), sym_1875);
    sym_352((&(((*(sym_1873)))._f617)), ((uint8_t)0));
    sym_394((&(((*(sym_1873)))._f587)), sym_236());
    sym_394((&(((*(sym_1873)))._f618)), sym_1874);
    sym_298(sym_1873, sym_1874, sym_1875);
    return sym_1875;
}

void sym_298(S_55* sym_1876, uint32_t sym_1877, uint32_t sym_1878) {
    uint64_t sym_1879 = ((uint64_t)sym_1877);
    while ((sym_1879 >= (((*(sym_1876)))._f619)._f567)) {
        sym_394((&(((*(sym_1876)))._f619)), sym_294());
    }
    sym_397((&(((*(sym_1876)))._f619)), sym_1879, sym_1878);
    return;
}

uint32_t sym_304(S_55* sym_1880, S_54* sym_1881, uint32_t sym_1882, uint32_t sym_1883) {
    uint32_t sym_1884 = sym_302(sym_1880, sym_1882);
    uint32_t sym_1885 = sym_396((&(((*(sym_1880)))._f587)), ((uint64_t)sym_1884));
    uint32_t sym_1886 = sym_1883;
    if ((sym_1885 != sym_236())) {
        sym_1886 = sym_305(sym_1880, sym_1881, sym_1885, sym_1883);
    }
    sym_397((&(((*(sym_1880)))._f587)), ((uint64_t)sym_1884), sym_1886);
    return sym_1886;
}

uint32_t sym_302(S_55* sym_1887, uint32_t sym_1888) {
    uint32_t sym_1889 = sym_1888;
    while (true) {
        uint32_t sym_1890 = sym_396((&(((*(sym_1887)))._f616)), ((uint64_t)sym_1889));
        if ((sym_1890 == sym_1889)) {
            return sym_1889;
        }
        uint32_t sym_1891 = sym_396((&(((*(sym_1887)))._f616)), ((uint64_t)sym_1890));
        sym_397((&(((*(sym_1887)))._f616)), ((uint64_t)sym_1889), sym_1891);
        sym_1889 = sym_1891;
    }
    return sym_1889;
}

uint32_t sym_305(S_55* sym_1892, S_54* sym_1893, uint32_t sym_1894, uint32_t sym_1895) {
    if ((sym_1894 == sym_1895)) {
        return sym_1894;
    }
    S_112 sym_1896 = sym_234();
    uint8_t sym_1897 = sym_260(sym_1893, sym_1894);
    uint8_t sym_1898 = sym_260(sym_1893, sym_1895);
    if (((sym_1897 == (sym_1896)._f645) && (sym_1898 == (sym_1896)._f645))) {
        uint32_t sym_1899 = sym_282(sym_1893, sym_1894);
        uint32_t sym_1900 = sym_282(sym_1893, sym_1895);
        uint32_t sym_1901 = sym_303(sym_1892, sym_1893, sym_1899, sym_1900);
        return sym_273(sym_1893, sym_1901);
    } else {
        if ((sym_1897 == (sym_1896)._f645)) {
            uint32_t sym_1902 = sym_282(sym_1893, sym_1894);
            sym_304(sym_1892, sym_1893, sym_1902, sym_1895);
            return sym_1895;
        } else {
            if ((sym_1898 == (sym_1896)._f645)) {
                uint32_t sym_1903 = sym_282(sym_1893, sym_1895);
                sym_304(sym_1892, sym_1893, sym_1903, sym_1894);
                return sym_1894;
            }
        }
    }
    if ((sym_1897 == (sym_1896)._f640)) {
        return sym_1895;
    }
    if ((sym_1898 == (sym_1896)._f640)) {
        return sym_1894;
    }
    if ((sym_1897 == (sym_1896)._f636)) {
        return sym_1895;
    }
    if ((sym_1898 == (sym_1896)._f636)) {
        return sym_1894;
    }
    bool sym_1904 = ((((sym_1898 == (sym_1896)._f646) || (sym_1898 == (sym_1896)._f647)) || (sym_1898 == (sym_1896)._f648)) || (sym_1898 == (sym_1896)._f638));
    if (((sym_1897 == (sym_1896)._f637) && sym_1904)) {
        return sym_1895;
    }
    bool sym_1905 = ((((sym_1897 == (sym_1896)._f646) || (sym_1897 == (sym_1896)._f647)) || (sym_1897 == (sym_1896)._f648)) || (sym_1897 == (sym_1896)._f638));
    if ((sym_1905 && (sym_1898 == (sym_1896)._f637))) {
        return sym_1894;
    }
    if (((sym_1897 == (sym_1896)._f638) && (sym_1898 == (sym_1896)._f648))) {
        return sym_1895;
    }
    if (((sym_1897 == (sym_1896)._f648) && (sym_1898 == (sym_1896)._f638))) {
        return sym_1894;
    }
    if (((sym_1897 == (sym_1896)._f566) && (sym_1898 == (sym_1896)._f566))) {
        uint32_t sym_1906 = sym_261(sym_1893, sym_1894);
        uint32_t sym_1907 = sym_261(sym_1893, sym_1895);
        uint32_t sym_1908 = sym_305(sym_1892, sym_1893, sym_1906, sym_1907);
        if ((sym_1908 == sym_244())) {
            return sym_244();
        }
        return sym_277(sym_1893, sym_1908);
    }
    if (((sym_1897 == (sym_1896)._f651) && (sym_1898 == (sym_1896)._f651))) {
        uint32_t sym_1909 = sym_284(sym_1893, sym_1894);
        uint32_t sym_1910 = sym_284(sym_1893, sym_1895);
        if ((sym_1909 != sym_1910)) {
            return sym_244();
        }
        uint32_t sym_1911 = sym_283(sym_1893, sym_1894);
        uint32_t sym_1912 = sym_283(sym_1893, sym_1895);
        uint32_t sym_1913 = sym_305(sym_1892, sym_1893, sym_1911, sym_1912);
        if ((sym_1913 == sym_244())) {
            return sym_244();
        }
        return sym_278(sym_1893, sym_1913, sym_1909);
    }
    if (((sym_1897 == (sym_1896)._f650) && (sym_1898 == (sym_1896)._f650))) {
        S_430 sym_1914 = sym_290(sym_1893, sym_1894);
        S_430 sym_1915 = sym_290(sym_1893, sym_1895);
        if (((sym_1914)._f567 != (sym_1915)._f567)) {
            return sym_244();
        }
        S_45 sym_1916 = sym_390();
        bool sym_1917 = false;
        uint64_t sym_1918 = ((sym_1914)._f567 / 2);
        uint64_t sym_1919 = 0;
        while ((sym_1919 < sym_1918)) {
            uint64_t sym_1920 = (sym_1919 * 2);
            uint32_t sym_1921 = (*(((sym_1914)._f566 + sym_1920)));
            uint32_t sym_1922 = (*((((sym_1914)._f566 + sym_1920) + 1)));
            uint32_t sym_1923 = (*(((sym_1915)._f566 + sym_1920)));
            uint32_t sym_1924 = (*((((sym_1915)._f566 + sym_1920) + 1)));
            if ((sym_1921 != sym_1923)) {
                sym_1917 = true;
                break;
            }
            uint32_t sym_1925 = sym_305(sym_1892, sym_1893, sym_1922, sym_1924);
            if ((sym_1925 == sym_244())) {
                sym_1917 = true;
                break;
            }
            sym_394((&(sym_1916)), sym_1921);
            sym_394((&(sym_1916)), sym_1925);
            sym_1919 = (sym_1919 + 1);
        }
        if (sym_1917) {
            sym_392((&(sym_1916)));
            return sym_244();
        }
        S_430 sym_1926 = ((S_430){._f566 = sym_404((&(sym_1916)), 0), ._f567 = (sym_1914)._f567});
        uint32_t sym_1927 = sym_281(sym_1893, sym_1926);
        sym_392((&(sym_1916)));
        return sym_1927;
    }
    return sym_244();
}

uint8_t sym_260(S_54* sym_1928, uint32_t sym_1929) {
    if ((sym_1929 == sym_236())) {
        return (sym_234())._f636;
    }
    return sym_354((&((((*(sym_1928)))._f612)._f606)), ((uint64_t)sym_1929));
}

uint32_t sym_282(S_54* sym_1930, uint32_t sym_1931) {
    return sym_261(sym_1930, sym_1931);
}

uint32_t sym_261(S_54* sym_1932, uint32_t sym_1933) {
    return sym_396((&((((*(sym_1932)))._f612)._f615)), ((uint64_t)sym_1933));
}

uint32_t sym_303(S_55* sym_1934, S_54* sym_1935, uint32_t sym_1936, uint32_t sym_1937) {
    uint32_t sym_1938 = sym_302(sym_1934, sym_1936);
    uint32_t sym_1939 = sym_302(sym_1934, sym_1937);
    if ((sym_1938 == sym_1939)) {
        return sym_1938;
    }
    uint32_t sym_1940 = sym_396((&(((*(sym_1934)))._f587)), ((uint64_t)sym_1938));
    uint32_t sym_1941 = sym_396((&(((*(sym_1934)))._f587)), ((uint64_t)sym_1939));
    if (((sym_1940 == sym_243()) || (sym_1941 == sym_243()))) {
        return sym_1938;
    }
    uint32_t sym_1942 = sym_236();
    bool sym_1943 = (sym_1940 != sym_236());
    bool sym_1944 = (sym_1941 != sym_236());
    if ((sym_1943 && sym_1944)) {
        sym_1942 = sym_305(sym_1934, sym_1935, sym_1940, sym_1941);
    } else {
        if (sym_1943) {
            sym_1942 = sym_1940;
        } else {
            if (sym_1944) {
                sym_1942 = sym_1941;
            }
        }
    }
    uint8_t sym_1945 = sym_354((&(((*(sym_1934)))._f617)), ((uint64_t)sym_1938));
    uint8_t sym_1946 = sym_354((&(((*(sym_1934)))._f617)), ((uint64_t)sym_1939));
    uint32_t sym_1947 = sym_1938;
    uint32_t sym_1948 = sym_1939;
    if ((sym_1945 > sym_1946)) {
        sym_1947 = sym_1938;
        sym_1948 = sym_1939;
    } else {
        if ((sym_1945 < sym_1946)) {
            sym_1947 = sym_1939;
            sym_1948 = sym_1938;
        } else {
            sym_355((&(((*(sym_1934)))._f617)), ((uint64_t)sym_1938), (sym_1945 + ((uint8_t)1)));
            sym_1947 = sym_1938;
            sym_1948 = sym_1939;
        }
    }
    sym_397((&(((*(sym_1934)))._f616)), ((uint64_t)sym_1948), sym_1947);
    if ((sym_1942 != sym_236())) {
        sym_397((&(((*(sym_1934)))._f587)), ((uint64_t)sym_1947), sym_1942);
    }
    return sym_1947;
}

uint32_t sym_243(void) {
    return 6;
}

void sym_355(S_42* sym_1949, uint64_t sym_1950, uint8_t sym_1951) {
    sym_375(sym_1949, sym_1950, sym_1951);
    return;
}

void sym_375(S_42* sym_1952, uint64_t sym_1953, uint8_t sym_1954) {
    (*((((*(sym_1952)))._f566 + sym_1953))) = sym_1954;
    return;
}

uint32_t sym_273(S_54* sym_1955, uint32_t sym_1956) {
    return sym_272(sym_1955, (sym_234())._f645, sym_1956);
}

uint32_t sym_272(S_54* sym_1957, uint8_t sym_1958, uint32_t sym_1959) {
    if ((((((*(sym_1957)))._f614 + 1) * 10) >= (((*(sym_1957)))._f613 * 7))) {
        sym_263(sym_1957);
    }
    uint32_t sym_1960 = sym_251(2166136261, ((uint32_t)sym_1958));
    sym_1960 = sym_251(sym_1960, sym_1959);
    uint32_t sym_1961 = (((*(sym_1957)))._f613 - 1);
    uint32_t sym_1962 = (sym_1960 & sym_1961);
    while (true) {
        uint32_t sym_1963 = sym_396((&(((*(sym_1957)))._f611)), ((uint64_t)sym_1962));
        if ((sym_1963 == sym_236())) {
            uint32_t sym_1964 = sym_250((&(((*(sym_1957)))._f612)), sym_1958, sym_1959);
            sym_397((&(((*(sym_1957)))._f611)), ((uint64_t)sym_1962), sym_1964);
            ((*(sym_1957)))._f614 = (((*(sym_1957)))._f614 + 1);
            return sym_1964;
        }
        if (sym_252((&(((*(sym_1957)))._f612)), sym_1963, sym_1958, sym_1959)) {
            return sym_1963;
        }
        sym_1962 = ((sym_1962 + 1) & sym_1961);
    }
    return sym_244();
}

void sym_263(S_54* sym_1965) {
    uint32_t sym_1966 = ((*(sym_1965)))._f613;
    uint32_t sym_1967 = (sym_1966 * 2);
    S_45 sym_1968 = sym_391(((uint64_t)sym_1967));
    uint32_t sym_1969 = 0;
    while ((sym_1969 < sym_1967)) {
        sym_394((&(sym_1968)), sym_236());
        sym_1969 = (sym_1969 + 1);
    }
    uint32_t sym_1970 = (sym_1967 - 1);
    uint32_t sym_1971 = sym_249((&(((*(sym_1965)))._f612)));
    uint32_t sym_1972 = sym_245();
    while ((sym_1972 < sym_1971)) {
        uint32_t sym_1973 = sym_262(sym_1965, sym_1972);
        uint32_t sym_1974 = (sym_1973 & sym_1970);
        while ((sym_396((&(sym_1968)), ((uint64_t)sym_1974)) != sym_236())) {
            sym_1974 = ((sym_1974 + 1) & sym_1970);
        }
        sym_397((&(sym_1968)), ((uint64_t)sym_1974), sym_1972);
        sym_1972 = (sym_1972 + 1);
    }
    sym_392((&(((*(sym_1965)))._f611)));
    ((*(sym_1965)))._f611 = sym_1968;
    ((*(sym_1965)))._f613 = sym_1967;
    return;
}

uint32_t sym_249(S_53* sym_1975) {
    return ((uint32_t)(((*(sym_1975)))._f606)._f567);
}

uint32_t sym_245(void) {
    return 8;
}

uint32_t sym_262(S_54* sym_1976, uint32_t sym_1977) {
    S_112 sym_1978 = sym_234();
    uint8_t sym_1979 = sym_260(sym_1976, sym_1977);
    uint32_t sym_1980 = sym_261(sym_1976, sym_1977);
    if ((sym_1979 == (sym_1978)._f651)) {
        uint64_t sym_1981 = ((uint64_t)sym_1980);
        uint32_t sym_1982 = sym_396((&((((*(sym_1976)))._f612)._f609)), sym_1981);
        uint32_t sym_1983 = sym_396((&((((*(sym_1976)))._f612)._f609)), (sym_1981 + 1));
        uint32_t sym_1984 = sym_251(2166136261, ((uint32_t)sym_1979));
        sym_1984 = sym_251(sym_1984, sym_1982);
        return sym_251(sym_1984, sym_1983);
    }
    if ((sym_1979 == (sym_1978)._f649)) {
        uint64_t sym_1985 = ((uint64_t)sym_1980);
        uint32_t sym_1986 = sym_396((&((((*(sym_1976)))._f612)._f609)), sym_1985);
        uint32_t sym_1987 = sym_396((&((((*(sym_1976)))._f612)._f609)), (sym_1985 + 1));
        uint32_t sym_1988 = (sym_1986 >> 16);
        uint32_t sym_1989 = sym_251(2166136261, ((uint32_t)sym_1979));
        sym_1989 = sym_251(sym_1989, sym_1987);
        sym_1989 = sym_251(sym_1989, sym_1986);
        uint64_t sym_1990 = 0;
        while (((sym_1990 < ((uint64_t)sym_1988)) || false)) {
            uint32_t sym_1991 = sym_396((&((((*(sym_1976)))._f612)._f609)), ((sym_1985 + 2) + sym_1990));
            sym_1989 = sym_251(sym_1989, sym_1991);
            sym_1990 = (sym_1990 + 1);
        }
        return sym_1989;
    }
    if ((sym_1979 == (sym_1978)._f650)) {
        uint64_t sym_1992 = ((uint64_t)sym_1980);
        uint32_t sym_1993 = sym_396((&((((*(sym_1976)))._f612)._f609)), sym_1992);
        uint64_t sym_1994 = ((uint64_t)(sym_1993 * 2));
        uint32_t sym_1995 = sym_251(2166136261, ((uint32_t)sym_1979));
        sym_1995 = sym_251(sym_1995, sym_1993);
        uint64_t sym_1996 = 0;
        while ((sym_1996 < sym_1994)) {
            uint32_t sym_1997 = sym_396((&((((*(sym_1976)))._f612)._f609)), ((sym_1992 + 1) + sym_1996));
            sym_1995 = sym_251(sym_1995, sym_1997);
            sym_1996 = (sym_1996 + 1);
        }
        return sym_1995;
    }
    uint32_t sym_1998 = sym_251(2166136261, ((uint32_t)sym_1979));
    return sym_251(sym_1998, sym_1980);
}

uint32_t sym_251(uint32_t sym_1999, uint32_t sym_2000) {
    return ((sym_1999 ^ sym_2000) * 16777619);
}

bool sym_252(S_53* sym_2001, uint32_t sym_2002, uint8_t sym_2003, uint32_t sym_2004) {
    if ((sym_354((&(((*(sym_2001)))._f606)), ((uint64_t)sym_2002)) != sym_2003)) {
        return false;
    }
    return (sym_396((&(((*(sym_2001)))._f615)), ((uint64_t)sym_2002)) == sym_2004);
}

uint32_t sym_244(void) {
    return 7;
}

uint32_t sym_277(S_54* sym_2005, uint32_t sym_2006) {
    return sym_272(sym_2005, (sym_234())._f566, sym_2006);
}

uint32_t sym_284(S_54* sym_2007, uint32_t sym_2008) {
    uint64_t sym_2009 = ((uint64_t)sym_261(sym_2007, sym_2008));
    return sym_396((&((((*(sym_2007)))._f612)._f609)), (sym_2009 + 1));
}

uint32_t sym_283(S_54* sym_2010, uint32_t sym_2011) {
    uint64_t sym_2012 = ((uint64_t)sym_261(sym_2010, sym_2011));
    return sym_396((&((((*(sym_2010)))._f612)._f609)), sym_2012);
}

uint32_t sym_278(S_54* sym_2013, uint32_t sym_2014, uint32_t sym_2015) {
    S_112 sym_2016 = sym_234();
    if ((((((*(sym_2013)))._f614 + 1) * 10) >= (((*(sym_2013)))._f613 * 7))) {
        sym_263(sym_2013);
    }
    uint32_t sym_2017 = sym_251(2166136261, ((uint32_t)(sym_2016)._f651));
    sym_2017 = sym_251(sym_2017, sym_2014);
    sym_2017 = sym_251(sym_2017, sym_2015);
    uint32_t sym_2018 = (((*(sym_2013)))._f613 - 1);
    uint32_t sym_2019 = (sym_2017 & sym_2018);
    while (true) {
        uint32_t sym_2020 = sym_396((&(((*(sym_2013)))._f611)), ((uint64_t)sym_2019));
        if ((sym_2020 == sym_236())) {
            uint32_t sym_2021 = ((uint32_t)((((*(sym_2013)))._f612)._f609)._f567);
            sym_394((&((((*(sym_2013)))._f612)._f609)), sym_2014);
            sym_394((&((((*(sym_2013)))._f612)._f609)), sym_2015);
            uint32_t sym_2022 = sym_250((&(((*(sym_2013)))._f612)), (sym_2016)._f651, sym_2021);
            sym_397((&(((*(sym_2013)))._f611)), ((uint64_t)sym_2019), sym_2022);
            ((*(sym_2013)))._f614 = (((*(sym_2013)))._f614 + 1);
            return sym_2022;
        }
        if (sym_253((&(((*(sym_2013)))._f612)), sym_2020, (sym_2016)._f651, sym_2014, sym_2015)) {
            return sym_2020;
        }
        sym_2019 = ((sym_2019 + 1) & sym_2018);
    }
    return sym_244();
}

bool sym_253(S_53* sym_2023, uint32_t sym_2024, uint8_t sym_2025, uint32_t sym_2026, uint32_t sym_2027) {
    if ((sym_354((&(((*(sym_2023)))._f606)), ((uint64_t)sym_2024)) != sym_2025)) {
        return false;
    }
    uint64_t sym_2028 = ((uint64_t)sym_396((&(((*(sym_2023)))._f615)), ((uint64_t)sym_2024)));
    uint32_t sym_2029 = sym_396((&(((*(sym_2023)))._f609)), sym_2028);
    uint32_t sym_2030 = sym_396((&(((*(sym_2023)))._f609)), (sym_2028 + 1));
    return ((sym_2029 == sym_2026) && (sym_2030 == sym_2027));
}

S_430 sym_290(S_54* sym_2031, uint32_t sym_2032) {
    uint64_t sym_2033 = ((uint64_t)sym_261(sym_2031, sym_2032));
    uint32_t sym_2034 = sym_396((&((((*(sym_2031)))._f612)._f609)), sym_2033);
    uint32_t* sym_2035 = sym_404((&((((*(sym_2031)))._f612)._f609)), (sym_2033 + 1));
    return ((S_430){._f566 = sym_2035, ._f567 = ((uint64_t)(sym_2034 * 2))});
}

uint32_t sym_281(S_54* sym_2036, S_430 sym_2037) {
    sym_280(sym_2037);
    S_112 sym_2038 = sym_234();
    if ((((((*(sym_2036)))._f614 + 1) * 10) >= (((*(sym_2036)))._f613 * 7))) {
        sym_263(sym_2036);
    }
    uint32_t sym_2039 = ((uint32_t)((sym_2037)._f567 / 2));
    uint32_t sym_2040 = sym_251(2166136261, ((uint32_t)(sym_2038)._f650));
    sym_2040 = sym_251(sym_2040, sym_2039);
    uint64_t sym_2041 = 0;
    while ((sym_2041 < (sym_2037)._f567)) {
        sym_2040 = sym_251(sym_2040, (*(((sym_2037)._f566 + sym_2041))));
        sym_2041 = (sym_2041 + 1);
    }
    uint32_t sym_2042 = (((*(sym_2036)))._f613 - 1);
    uint32_t sym_2043 = (sym_2040 & sym_2042);
    while (true) {
        uint32_t sym_2044 = sym_396((&(((*(sym_2036)))._f611)), ((uint64_t)sym_2043));
        if ((sym_2044 == sym_236())) {
            uint32_t sym_2045 = ((uint32_t)((((*(sym_2036)))._f612)._f609)._f567);
            sym_394((&((((*(sym_2036)))._f612)._f609)), sym_2039);
            uint64_t sym_2046 = 0;
            while ((sym_2046 < (sym_2037)._f567)) {
                sym_394((&((((*(sym_2036)))._f612)._f609)), (*(((sym_2037)._f566 + sym_2046))));
                sym_2046 = (sym_2046 + 1);
            }
            uint32_t sym_2047 = sym_250((&(((*(sym_2036)))._f612)), (sym_2038)._f650, sym_2045);
            sym_397((&(((*(sym_2036)))._f611)), ((uint64_t)sym_2043), sym_2047);
            ((*(sym_2036)))._f614 = (((*(sym_2036)))._f614 + 1);
            return sym_2047;
        }
        if (sym_255((&(((*(sym_2036)))._f612)), sym_2044, (sym_2038)._f650, sym_2037, sym_2039)) {
            return sym_2044;
        }
        sym_2043 = ((sym_2043 + 1) & sym_2042);
    }
    return sym_244();
}

void sym_280(S_430 sym_2048) {
    uint64_t sym_2049 = ((sym_2048)._f567 / 2);
    if ((sym_2049 <= 1)) {
        return;
    }
    uint64_t sym_2050 = 0;
    while ((sym_2050 < sym_2049)) {
        uint64_t sym_2051 = (sym_2050 + 1);
        while ((sym_2051 < sym_2049)) {
            uint64_t sym_2052 = (sym_2050 * 2);
            uint64_t sym_2053 = (sym_2051 * 2);
            uint32_t sym_2054 = (*(((sym_2048)._f566 + sym_2052)));
            uint32_t sym_2055 = (*(((sym_2048)._f566 + sym_2053)));
            if ((sym_2054 > sym_2055)) {
                uint32_t sym_2056 = (*((((sym_2048)._f566 + sym_2052) + 1)));
                uint32_t sym_2057 = (*((((sym_2048)._f566 + sym_2053) + 1)));
                (*(((sym_2048)._f566 + sym_2052))) = sym_2055;
                (*((((sym_2048)._f566 + sym_2052) + 1))) = sym_2057;
                (*(((sym_2048)._f566 + sym_2053))) = sym_2054;
                (*((((sym_2048)._f566 + sym_2053) + 1))) = sym_2056;
            }
            sym_2051 = (sym_2051 + 1);
        }
        sym_2050 = (sym_2050 + 1);
    }
    return;
}

bool sym_255(S_53* sym_2058, uint32_t sym_2059, uint8_t sym_2060, S_430 sym_2061, uint32_t sym_2062) {
    if ((sym_354((&(((*(sym_2058)))._f606)), ((uint64_t)sym_2059)) != sym_2060)) {
        return false;
    }
    uint64_t sym_2063 = ((uint64_t)sym_396((&(((*(sym_2058)))._f615)), ((uint64_t)sym_2059)));
    uint32_t sym_2064 = sym_396((&(((*(sym_2058)))._f609)), sym_2063);
    if ((sym_2064 != sym_2062)) {
        return false;
    }
    uint64_t sym_2065 = 0;
    uint64_t sym_2066 = ((uint64_t)(sym_2062 * 2));
    while ((sym_2065 < sym_2066)) {
        uint32_t sym_2067 = sym_396((&(((*(sym_2058)))._f609)), ((sym_2063 + 1) + sym_2065));
        if (((sym_2067 != (*(((sym_2061)._f566 + sym_2065)))) || false)) {
            return false;
        }
        sym_2065 = (sym_2065 + 1);
    }
    return true;
}

uint32_t sym_238(void) {
    return 1;
}

void sym_162(S_59* sym_2068, uint32_t sym_2069) {
    S_57* sym_2070 = (&(((*(sym_2068)))._f579));
    S_51* sym_2071 = (&(((*(sym_2070)))._f583));
    S_55* sym_2072 = (&(((*(sym_2070)))._f588));
    S_54* sym_2073 = (&(((*(sym_2070)))._f587));
    uint32_t sym_2074 = sym_95(sym_2071, sym_2069);
    S_23 sym_2075 = sym_212((&(((*(sym_2070)))._f586)), sym_2074);
    uint32_t sym_2076 = sym_301(sym_2072, sym_2069);
    if ((sym_2076 == sym_294())) {
        sym_2076 = sym_299(sym_2072, sym_2069);
    }
    uint32_t sym_2077 = sym_158(sym_2073, sym_2075);
    if ((sym_2077 != sym_236())) {
        sym_304(sym_2072, sym_2073, sym_2076, sym_241());
        sym_226(sym_2070, sym_2069, sym_2077);
        return;
    }
    uint32_t sym_2078 = sym_225(sym_2070, sym_2069);
    if ((sym_2078 != sym_532())) {
        uint32_t sym_2079 = sym_223(sym_2070, sym_2078);
        if ((sym_2079 == sym_2069)) {
            return;
        }
        uint32_t sym_2080 = sym_318(sym_2068, sym_2078);
        if ((sym_2080 != sym_294())) {
            sym_303(sym_2072, sym_2073, sym_2076, sym_2080);
        }
        uint32_t sym_2081 = sym_223(sym_2070, sym_2078);
        if ((sym_2081 != sym_532())) {
            uint32_t sym_2082 = sym_227(sym_2070, sym_2081);
            if ((sym_2082 != sym_236())) {
                sym_226(sym_2070, sym_2069, sym_2082);
            }
        }
    }
    return;
}

uint32_t sym_301(S_55* sym_2083, uint32_t sym_2084) {
    uint64_t sym_2085 = ((uint64_t)sym_2084);
    if ((sym_2085 >= (((*(sym_2083)))._f619)._f567)) {
        return sym_294();
    }
    return sym_396((&(((*(sym_2083)))._f619)), sym_2085);
}

uint32_t sym_158(S_54* sym_2086, S_23 sym_2087) {
    if (sym_157(sym_2087, sym_50(((uint8_t*)"bool")))) {
        return sym_267();
    }
    if (sym_157(sym_2087, sym_50(((uint8_t*)"unit")))) {
        return sym_269();
    }
    if (((sym_2087)._f567 < 2)) {
        return sym_236();
    }
    uint8_t sym_2088 = (*((sym_2087)._f566));
    if ((((sym_2088 != 105) && (sym_2088 != 117)) && (sym_2088 != 102))) {
        return sym_236();
    }
    uint32_t sym_2089 = 0;
    uint64_t sym_2090 = 1;
    while ((sym_2090 < (sym_2087)._f567)) {
        uint8_t sym_2091 = (*(((sym_2087)._f566 + sym_2090)));
        if (((sym_2091 < 48) || (sym_2091 > 57))) {
            return sym_236();
        }
        sym_2089 = ((sym_2089 * 10) + ((uint32_t)(sym_2091 - 48)));
        sym_2090 = (sym_2090 + 1);
    }
    if ((((((sym_2089 != 8) && (sym_2089 != 16)) && (sym_2089 != 32)) && (sym_2089 != 64)) && (sym_2089 != 128))) {
        return sym_236();
    }
    if ((sym_2088 == 105)) {
        return sym_274(sym_2086, ((uint16_t)sym_2089));
    }
    if ((sym_2088 == 117)) {
        return sym_275(sym_2086, ((uint16_t)sym_2089));
    }
    if ((sym_2088 == 102)) {
        return sym_276(sym_2086, ((uint16_t)sym_2089));
    }
    return sym_236();
}

bool sym_157(S_23 sym_2092, S_23 sym_2093) {
    if (((sym_2092)._f567 != (sym_2093)._f567)) {
        return false;
    }
    uint64_t sym_2094 = 0;
    while ((sym_2094 < (sym_2092)._f567)) {
        if (((*(((sym_2092)._f566 + sym_2094))) != (*(((sym_2093)._f566 + sym_2094))))) {
            return false;
        }
        sym_2094 = (sym_2094 + 1);
    }
    return true;
}

uint32_t sym_267(void) {
    return sym_240();
}

uint32_t sym_240(void) {
    return 3;
}

uint32_t sym_269(void) {
    return sym_242();
}

uint32_t sym_242(void) {
    return 5;
}

uint32_t sym_274(S_54* sym_2095, uint16_t sym_2096) {
    return sym_272(sym_2095, (sym_234())._f646, ((uint32_t)sym_2096));
}

uint32_t sym_275(S_54* sym_2097, uint16_t sym_2098) {
    return sym_272(sym_2097, (sym_234())._f647, ((uint32_t)sym_2098));
}

uint32_t sym_276(S_54* sym_2099, uint16_t sym_2100) {
    return sym_272(sym_2099, (sym_234())._f648, ((uint32_t)sym_2100));
}

uint32_t sym_241(void) {
    return 4;
}

void sym_226(S_57* sym_2101, uint32_t sym_2102, uint32_t sym_2103) {
    sym_541((&(((*(sym_2101)))._f598)), sym_2102, sym_2103, sym_236());
    return;
}

uint32_t sym_225(S_57* sym_2104, uint32_t sym_2105) {
    if ((sym_2105 == sym_532())) {
        return sym_532();
    }
    if ((sym_2105 >= sym_536((&(((*(sym_2104)))._f597))))) {
        return sym_532();
    }
    return sym_538((&(((*(sym_2104)))._f597)), sym_2105);
}

uint32_t sym_318(S_59* sym_2106, uint32_t sym_2107) {
    sym_317(sym_2106, sym_2107);
    uint32_t sym_2108 = sym_223((&(((*(sym_2106)))._f579)), sym_2107);
    if ((sym_2108 == sym_532())) {
        return sym_294();
    }
    return sym_301((&((((*(sym_2106)))._f579)._f588)), sym_2108);
}

void sym_317(S_59* sym_2109, uint32_t sym_2110) {
    S_57* sym_2111 = (&(((*(sym_2109)))._f579));
    uint32_t sym_2112 = sym_223(sym_2111, sym_2110);
    if ((sym_2112 != sym_532())) {
        uint32_t sym_2113 = sym_301((&(((*(sym_2111)))._f588)), sym_2112);
        if ((sym_2113 != sym_294())) {
            uint32_t sym_2114 = sym_306((&(((*(sym_2111)))._f588)), sym_2113);
            if ((sym_2114 != sym_236())) {
                return;
            }
        }
    }
    if (sym_229(sym_2111, sym_2110)) {
        return;
    }
    uint32_t sym_2115 = sym_538((&(((*(sym_2111)))._f593)), sym_2110);
    if ((sym_2115 == sym_532())) {
        sym_228(sym_2111, sym_2110, true);
        sym_315(sym_2109, sym_2110);
        sym_228(sym_2111, sym_2110, false);
    }
    sym_2112 = sym_223(sym_2111, sym_2110);
    if ((sym_2112 != sym_532())) {
        uint32_t sym_2116 = sym_301((&(((*(sym_2111)))._f588)), sym_2112);
        if ((sym_2116 == sym_294())) {
            sym_299((&(((*(sym_2111)))._f588)), sym_2112);
        }
    }
    sym_228(sym_2111, sym_2110, true);
    sym_316(sym_2109, sym_2110);
    sym_228(sym_2111, sym_2110, false);
    return;
}

uint32_t sym_306(S_55* sym_2117, uint32_t sym_2118) {
    uint32_t sym_2119 = sym_302(sym_2117, sym_2118);
    return sym_396((&(((*(sym_2117)))._f587)), ((uint64_t)sym_2119));
}

bool sym_229(S_57* sym_2120, uint32_t sym_2121) {
    if ((sym_2121 >= sym_536((&(((*(sym_2120)))._f599))))) {
        return false;
    }
    return (sym_538((&(((*(sym_2120)))._f599)), sym_2121) == 1);
}

void sym_228(S_57* sym_2122, uint32_t sym_2123, bool sym_2124) {
    uint32_t sym_2125 = 0;
    if (sym_2124) {
        sym_2125 = 1;
    }
    sym_541((&(((*(sym_2122)))._f599)), sym_2123, sym_2125, 0);
    return;
}

uint32_t sym_227(S_57* sym_2126, uint32_t sym_2127) {
    if ((sym_2127 >= sym_536((&(((*(sym_2126)))._f598))))) {
        return sym_236();
    }
    return sym_538((&(((*(sym_2126)))._f598)), sym_2127);
}

void sym_163(S_57* sym_2128, S_51* sym_2129, uint32_t sym_2130) {
    S_55* sym_2131 = (&(((*(sym_2128)))._f588));
    S_54* sym_2132 = (&(((*(sym_2128)))._f587));
    uint32_t sym_2133 = sym_95(sym_2129, sym_2130);
    uint32_t sym_2134 = sym_96(sym_2129, sym_2130);
    uint32_t sym_2135 = sym_97(sym_2129, sym_2130);
    uint32_t sym_2136 = sym_299(sym_2131, sym_2130);
    uint32_t sym_2137 = sym_301(sym_2131, sym_2133);
    if ((sym_2137 == sym_294())) {
        sym_2137 = sym_299(sym_2131, sym_2133);
    }
    if ((sym_2134 != sym_85())) {
        uint32_t sym_2138 = sym_301(sym_2131, sym_2134);
        if ((sym_2138 != sym_294())) {
            sym_304(sym_2131, sym_2132, sym_2138, sym_241());
        }
        uint32_t sym_2139 = sym_227(sym_2128, sym_2134);
        if ((sym_2139 != sym_236())) {
            sym_304(sym_2131, sym_2132, sym_2137, sym_2139);
            sym_226(sym_2128, sym_2133, sym_2139);
            sym_226(sym_2128, sym_2130, sym_2139);
        }
    }
    if ((sym_2135 != sym_85())) {
        uint32_t sym_2140 = sym_301(sym_2131, sym_2135);
        if ((sym_2140 != sym_294())) {
            uint32_t sym_2141 = sym_306(sym_2131, sym_2140);
            if ((sym_260(sym_2132, sym_2141) == (sym_234())._f639)) {
                uint32_t sym_2142 = sym_227(sym_2128, sym_2135);
                sym_226(sym_2128, sym_2133, sym_2142);
                sym_226(sym_2128, sym_2130, sym_2142);
            }
            sym_303(sym_2131, sym_2132, sym_2137, sym_2140);
        }
    }
    sym_303(sym_2131, sym_2132, sym_2136, sym_2137);
    return;
}

uint32_t sym_96(S_51* sym_2143, uint32_t sym_2144) {
    return sym_396((&(((*(sym_2143)))._f608)), ((uint64_t)sym_2144));
}

uint32_t sym_97(S_51* sym_2145, uint32_t sym_2146) {
    return sym_396((&(((*(sym_2145)))._f609)), ((uint64_t)sym_2146));
}

void sym_164(S_57* sym_2147, S_51* sym_2148, uint32_t sym_2149) {
    S_55* sym_2150 = (&(((*(sym_2147)))._f588));
    S_54* sym_2151 = (&(((*(sym_2147)))._f587));
    uint32_t sym_2152 = sym_95(sym_2148, sym_2149);
    uint32_t sym_2153 = sym_96(sym_2148, sym_2149);
    uint32_t sym_2154 = sym_301(sym_2150, sym_2152);
    if ((sym_2154 == sym_294())) {
        sym_2154 = sym_299(sym_2150, sym_2152);
    }
    if ((sym_2153 != sym_85())) {
        uint32_t sym_2155 = sym_227(sym_2147, sym_2153);
        if ((sym_2155 != sym_236())) {
            sym_304(sym_2150, sym_2151, sym_2154, sym_2155);
            sym_226(sym_2147, sym_2152, sym_2155);
            sym_226(sym_2147, sym_2149, sym_2155);
        }
    }
    return;
}

void sym_165(S_57* sym_2156, S_51* sym_2157, uint32_t sym_2158, uint32_t sym_2159) {
    S_55* sym_2160 = (&(((*(sym_2156)))._f588));
    S_54* sym_2161 = (&(((*(sym_2156)))._f587));
    uint32_t sym_2162 = sym_299(sym_2160, sym_2158);
    uint32_t sym_2163 = sym_95(sym_2157, sym_2158);
    uint32_t sym_2164 = sym_96(sym_2157, sym_2158);
    uint32_t sym_2165 = sym_97(sym_2157, sym_2158);
    bool sym_2166 = (sym_2165 == sym_85());
    uint32_t sym_2167 = sym_242();
    if ((sym_2164 != sym_85())) {
        uint32_t sym_2168 = sym_227(sym_2156, sym_2164);
        if ((sym_2168 != sym_236())) {
            sym_2167 = sym_2168;
        }
    }
    if ((sym_2159 != sym_294())) {
        sym_304(sym_2160, sym_2161, sym_2159, sym_2167);
    }
    S_45 sym_2169 = sym_390();
    if ((sym_2163 != sym_85())) {
        S_430 sym_2170 = sym_217(sym_2156, sym_2163);
        uint64_t sym_2171 = 0;
        while ((sym_2171 < (sym_2170)._f567)) {
            uint32_t sym_2172 = (*(((sym_2170)._f566 + sym_2171)));
            uint32_t sym_2173 = sym_96(sym_2157, sym_2172);
            uint32_t sym_2174 = sym_227(sym_2156, sym_2173);
            if ((sym_2174 == sym_236())) {
                uint32_t sym_2175 = sym_95(sym_2157, sym_2172);
                uint32_t sym_2176 = sym_301(sym_2160, sym_2175);
                if ((sym_2176 != sym_294())) {
                    sym_2174 = sym_306(sym_2160, sym_2176);
                }
            }
            if ((sym_2174 == sym_236())) {
                sym_2174 = sym_237();
            }
            sym_394((&(sym_2169)), sym_2174);
            sym_2171 = (sym_2171 + 1);
        }
    }
    S_430 sym_2177 = ((S_430){._f566 = ((uint32_t*)0), ._f567 = 0});
    if (((sym_2169)._f567 > 0)) {
        (sym_2177)._f566 = sym_404((&(sym_2169)), 0);
        (sym_2177)._f567 = (sym_2169)._f567;
    }
    uint32_t sym_2178 = sym_279(sym_2161, sym_2177, sym_2167, sym_2166);
    sym_392((&(sym_2169)));
    sym_304(sym_2160, sym_2161, sym_2162, sym_2178);
    if (((!(sym_2166)) && (sym_2165 != sym_85()))) {
        uint32_t sym_2179 = sym_301(sym_2160, sym_2165);
        if ((sym_2179 != sym_294())) {
            if (((sym_2159 != sym_294()) && (sym_2167 != sym_242()))) {
                sym_303(sym_2160, sym_2161, sym_2179, sym_2159);
            }
        }
    }
    return;
}

S_430 sym_217(S_57* sym_2180, uint32_t sym_2181) {
    S_45* sym_2182 = (&(((*(sym_2180)))._f585));
    uint32_t sym_2183 = sym_396(sym_2182, ((uint64_t)sym_2181));
    return ((S_430){._f566 = sym_404(sym_2182, (((uint64_t)sym_2181) + 1)), ._f567 = ((uint64_t)sym_2183)});
}

uint32_t sym_237(void) {
    return 0;
}

uint32_t sym_279(S_54* sym_2184, S_430 sym_2185, uint32_t sym_2186, bool sym_2187) {
    S_112 sym_2188 = sym_234();
    if ((((((*(sym_2184)))._f614 + 1) * 10) >= (((*(sym_2184)))._f613 * 7))) {
        sym_263(sym_2184);
    }
    uint32_t sym_2189 = ((uint32_t)(sym_2185)._f567);
    uint32_t sym_2190 = 0;
    if (sym_2187) {
        sym_2190 = 1;
    }
    uint32_t sym_2191 = ((sym_2189 << 16) | sym_2190);
    uint32_t sym_2192 = sym_251(2166136261, ((uint32_t)(sym_2188)._f649));
    sym_2192 = sym_251(sym_2192, sym_2186);
    sym_2192 = sym_251(sym_2192, sym_2191);
    uint64_t sym_2193 = 0;
    while ((sym_2193 < (sym_2185)._f567)) {
        sym_2192 = sym_251(sym_2192, (*(((sym_2185)._f566 + sym_2193))));
        sym_2193 = (sym_2193 + 1);
    }
    uint32_t sym_2194 = (((*(sym_2184)))._f613 - 1);
    uint32_t sym_2195 = (sym_2192 & sym_2194);
    while (true) {
        uint32_t sym_2196 = sym_396((&(((*(sym_2184)))._f611)), ((uint64_t)sym_2195));
        if ((sym_2196 == sym_236())) {
            uint32_t sym_2197 = ((uint32_t)((((*(sym_2184)))._f612)._f609)._f567);
            sym_394((&((((*(sym_2184)))._f612)._f609)), sym_2191);
            sym_394((&((((*(sym_2184)))._f612)._f609)), sym_2186);
            uint64_t sym_2198 = 0;
            while ((sym_2198 < (sym_2185)._f567)) {
                sym_394((&((((*(sym_2184)))._f612)._f609)), (*(((sym_2185)._f566 + sym_2198))));
                sym_2198 = (sym_2198 + 1);
            }
            uint32_t sym_2199 = sym_250((&(((*(sym_2184)))._f612)), (sym_2188)._f649, sym_2197);
            sym_397((&(((*(sym_2184)))._f611)), ((uint64_t)sym_2195), sym_2199);
            ((*(sym_2184)))._f614 = (((*(sym_2184)))._f614 + 1);
            return sym_2199;
        }
        if (sym_254((&(((*(sym_2184)))._f612)), sym_2196, (sym_2188)._f649, sym_2185, sym_2186, sym_2191)) {
            return sym_2196;
        }
        sym_2195 = ((sym_2195 + 1) & sym_2194);
    }
    return sym_244();
}

bool sym_254(S_53* sym_2200, uint32_t sym_2201, uint8_t sym_2202, S_430 sym_2203, uint32_t sym_2204, uint32_t sym_2205) {
    if ((sym_354((&(((*(sym_2200)))._f606)), ((uint64_t)sym_2201)) != sym_2202)) {
        return false;
    }
    uint64_t sym_2206 = ((uint64_t)sym_396((&(((*(sym_2200)))._f615)), ((uint64_t)sym_2201)));
    uint32_t sym_2207 = sym_396((&(((*(sym_2200)))._f609)), sym_2206);
    uint32_t sym_2208 = sym_396((&(((*(sym_2200)))._f609)), (sym_2206 + 1));
    if (((sym_2207 != sym_2205) || (sym_2208 != sym_2204))) {
        return false;
    }
    uint64_t sym_2209 = 0;
    while ((sym_2209 < (sym_2203)._f567)) {
        uint32_t sym_2210 = sym_396((&(((*(sym_2200)))._f609)), ((sym_2206 + 2) + sym_2209));
        if ((sym_2210 != (*(((sym_2203)._f566 + sym_2209))))) {
            return false;
        }
        sym_2209 = (sym_2209 + 1);
    }
    return true;
}

void sym_166(S_57* sym_2211, S_51* sym_2212, uint32_t sym_2213, uint32_t sym_2214) {
    S_55* sym_2215 = (&(((*(sym_2211)))._f588));
    S_54* sym_2216 = (&(((*(sym_2211)))._f587));
    uint32_t sym_2217 = sym_299(sym_2215, sym_2213);
    sym_304(sym_2215, sym_2216, sym_2217, sym_243());
    uint32_t sym_2218 = sym_95(sym_2212, sym_2213);
    if ((sym_2218 != sym_85())) {
        uint32_t sym_2219 = sym_301(sym_2215, sym_2218);
        if (((sym_2219 != sym_294()) && (sym_2214 != sym_294()))) {
            sym_303(sym_2215, sym_2216, sym_2219, sym_2214);
        }
    } else {
        if ((sym_2214 != sym_294())) {
            uint32_t sym_2220 = sym_300(sym_2215);
            sym_304(sym_2215, sym_2216, sym_2220, sym_242());
            sym_303(sym_2215, sym_2216, sym_2220, sym_2214);
        }
    }
    return;
}

void sym_167(S_57* sym_2221, S_51* sym_2222, uint32_t sym_2223) {
    S_55* sym_2224 = (&(((*(sym_2221)))._f588));
    S_54* sym_2225 = (&(((*(sym_2221)))._f587));
    uint32_t sym_2226 = sym_299(sym_2224, sym_2223);
    uint32_t sym_2227 = sym_95(sym_2222, sym_2223);
    uint32_t sym_2228 = sym_301(sym_2224, sym_2227);
    if ((sym_2228 != sym_294())) {
        uint32_t sym_2229 = sym_307(sym_2224, sym_2225, sym_2228);
        if ((sym_260(sym_2225, sym_2229) == (sym_234())._f649)) {
            uint32_t sym_2230 = sym_285(sym_2225, sym_2229);
            sym_304(sym_2224, sym_2225, sym_2226, sym_2230);
            uint32_t sym_2231 = sym_96(sym_2222, sym_2223);
            if ((sym_2231 != sym_85())) {
                S_430 sym_2232 = sym_217(sym_2221, sym_2231);
                S_430 sym_2233 = sym_288(sym_2225, sym_2229);
                uint64_t sym_2234 = 0;
                while (((sym_2234 < (sym_2232)._f567) && (sym_2234 < (sym_2233)._f567))) {
                    uint32_t sym_2235 = (*(((sym_2232)._f566 + sym_2234)));
                    uint32_t sym_2236 = sym_301(sym_2224, sym_2235);
                    uint32_t sym_2237 = (*(((sym_2233)._f566 + sym_2234)));
                    if ((sym_2236 != sym_294())) {
                        uint32_t sym_2238 = sym_300(sym_2224);
                        sym_304(sym_2224, sym_2225, sym_2238, sym_2237);
                        sym_303(sym_2224, sym_2225, sym_2236, sym_2238);
                    }
                    sym_2234 = (sym_2234 + 1);
                }
            }
        }
    }
    return;
}

uint32_t sym_307(S_55* sym_2239, S_54* sym_2240, uint32_t sym_2241) {
    uint32_t sym_2242 = sym_306(sym_2239, sym_2241);
    if ((sym_2242 == sym_236())) {
        return sym_236();
    }
    S_112 sym_2243 = sym_234();
    uint8_t sym_2244 = sym_260(sym_2240, sym_2242);
    if ((sym_2244 == (sym_2243)._f645)) {
        uint32_t sym_2245 = sym_282(sym_2240, sym_2242);
        return sym_307(sym_2239, sym_2240, sym_2245);
    }
    if ((sym_2244 == (sym_2243)._f566)) {
        uint32_t sym_2246 = sym_261(sym_2240, sym_2242);
        if ((sym_260(sym_2240, sym_2246) == (sym_2243)._f645)) {
            uint32_t sym_2247 = sym_282(sym_2240, sym_2246);
            uint32_t sym_2248 = sym_307(sym_2239, sym_2240, sym_2247);
            return sym_277(sym_2240, sym_2248);
        }
        return sym_2242;
    }
    if ((sym_2244 == (sym_2243)._f651)) {
        uint32_t sym_2249 = sym_284(sym_2240, sym_2242);
        uint32_t sym_2250 = sym_283(sym_2240, sym_2242);
        if ((sym_260(sym_2240, sym_2250) == (sym_2243)._f645)) {
            uint32_t sym_2251 = sym_282(sym_2240, sym_2250);
            uint32_t sym_2252 = sym_307(sym_2239, sym_2240, sym_2251);
            return sym_278(sym_2240, sym_2252, sym_2249);
        }
        return sym_2242;
    }
    if ((sym_2244 == (sym_2243)._f650)) {
        S_430 sym_2253 = sym_290(sym_2240, sym_2242);
        uint64_t sym_2254 = ((sym_2253)._f567 / 2);
        S_45 sym_2255 = sym_390();
        uint64_t sym_2256 = false;
        uint64_t sym_2257 = 0;
        while ((sym_2257 < sym_2254)) {
            uint64_t sym_2258 = (sym_2257 * 2);
            uint32_t sym_2259 = (*(((sym_2253)._f566 + sym_2258)));
            uint32_t sym_2260 = (*((((sym_2253)._f566 + sym_2258) + 1)));
            if ((sym_260(sym_2240, sym_2260) == (sym_2243)._f645)) {
                uint32_t sym_2261 = sym_282(sym_2240, sym_2260);
                uint32_t sym_2262 = sym_307(sym_2239, sym_2240, sym_2261);
                sym_394((&(sym_2255)), sym_2259);
                sym_394((&(sym_2255)), sym_2262);
                sym_2256 = true;
            } else {
                sym_394((&(sym_2255)), sym_2259);
                sym_394((&(sym_2255)), sym_2260);
            }
            sym_2257 = (sym_2257 + 1);
        }
        uint32_t sym_2263 = sym_2242;
        if (sym_2256) {
            S_430 sym_2264 = ((S_430){._f566 = sym_404((&(sym_2255)), 0), ._f567 = (sym_2253)._f567});
            sym_2263 = sym_281(sym_2240, sym_2264);
        }
        sym_392((&(sym_2255)));
        return sym_2263;
    }
    return sym_2242;
}

uint32_t sym_285(S_54* sym_2265, uint32_t sym_2266) {
    uint64_t sym_2267 = ((uint64_t)sym_261(sym_2265, sym_2266));
    return sym_396((&((((*(sym_2265)))._f612)._f609)), (sym_2267 + 1));
}

S_430 sym_288(S_54* sym_2268, uint32_t sym_2269) {
    uint64_t sym_2270 = ((uint64_t)sym_261(sym_2268, sym_2269));
    uint32_t sym_2271 = sym_286(sym_2268, sym_2269);
    uint32_t* sym_2272 = sym_404((&((((*(sym_2268)))._f612)._f609)), (sym_2270 + 2));
    return ((S_430){._f566 = sym_2272, ._f567 = ((uint64_t)sym_2271)});
}

uint32_t sym_286(S_54* sym_2273, uint32_t sym_2274) {
    uint64_t sym_2275 = ((uint64_t)sym_261(sym_2273, sym_2274));
    uint32_t sym_2276 = sym_396((&((((*(sym_2273)))._f612)._f609)), sym_2275);
    return (sym_2276 >> 16);
}

void sym_168(S_57* sym_2277, S_51* sym_2278, uint32_t sym_2279) {
    S_55* sym_2280 = (&(((*(sym_2277)))._f588));
    S_54* sym_2281 = (&(((*(sym_2277)))._f587));
    uint32_t sym_2282 = sym_299(sym_2280, sym_2279);
    uint32_t sym_2283 = sym_95(sym_2278, sym_2279);
    S_430 sym_2284 = sym_217(sym_2277, sym_2283);
    if (((sym_2284)._f567 > 0)) {
        uint32_t sym_2285 = (*((((sym_2284)._f566 + (sym_2284)._f567) - 1)));
        uint32_t sym_2286 = sym_301(sym_2280, sym_2285);
        if ((sym_2286 != sym_294())) {
            sym_303(sym_2280, sym_2281, sym_2282, sym_2286);
        }
    } else {
        sym_304(sym_2280, sym_2281, sym_2282, sym_242());
    }
    return;
}

void sym_169(S_57* sym_2287, S_51* sym_2288, uint32_t sym_2289) {
    S_55* sym_2290 = (&(((*(sym_2287)))._f588));
    S_54* sym_2291 = (&(((*(sym_2287)))._f587));
    uint32_t sym_2292 = sym_299(sym_2290, sym_2289);
    uint32_t sym_2293 = sym_95(sym_2288, sym_2289);
    uint32_t sym_2294 = sym_96(sym_2288, sym_2289);
    uint32_t sym_2295 = sym_301(sym_2290, sym_2293);
    uint32_t sym_2296 = sym_301(sym_2290, sym_2294);
    if (((sym_2295 != sym_294()) && (sym_2296 != sym_294()))) {
        uint32_t sym_2297 = sym_307(sym_2290, sym_2291, sym_2295);
        uint32_t sym_2298 = sym_307(sym_2290, sym_2291, sym_2296);
        uint8_t sym_2299 = sym_260(sym_2291, sym_2297);
        uint8_t sym_2300 = sym_260(sym_2291, sym_2298);
        if ((sym_2299 == (sym_234())._f566)) {
            sym_304(sym_2290, sym_2291, sym_2292, sym_2297);
            return;
        }
        if ((sym_2300 == (sym_234())._f566)) {
            sym_304(sym_2290, sym_2291, sym_2292, sym_2298);
            return;
        }
        sym_303(sym_2290, sym_2291, sym_2295, sym_2296);
        sym_303(sym_2290, sym_2291, sym_2292, sym_2295);
    }
    return;
}

void sym_170(S_57* sym_2301, S_51* sym_2302, uint32_t sym_2303) {
    S_55* sym_2304 = (&(((*(sym_2301)))._f588));
    S_54* sym_2305 = (&(((*(sym_2301)))._f587));
    uint32_t sym_2306 = sym_299(sym_2304, sym_2303);
    uint32_t sym_2307 = sym_95(sym_2302, sym_2303);
    uint32_t sym_2308 = sym_96(sym_2302, sym_2303);
    uint32_t sym_2309 = sym_301(sym_2304, sym_2307);
    uint32_t sym_2310 = sym_301(sym_2304, sym_2308);
    if (((sym_2309 != sym_294()) && (sym_2310 != sym_294()))) {
        sym_303(sym_2304, sym_2305, sym_2309, sym_2310);
        sym_303(sym_2304, sym_2305, sym_2306, sym_2309);
    }
    return;
}

void sym_171(S_57* sym_2311, S_51* sym_2312, uint32_t sym_2313) {
    uint32_t sym_2314 = sym_299((&(((*(sym_2311)))._f588)), sym_2313);
    sym_304((&(((*(sym_2311)))._f588)), (&(((*(sym_2311)))._f587)), sym_2314, sym_239());
    return;
}

uint32_t sym_239(void) {
    return 2;
}

void sym_172(S_57* sym_2315, S_51* sym_2316, uint32_t sym_2317) {
    uint32_t sym_2318 = sym_299((&(((*(sym_2315)))._f588)), sym_2317);
    sym_304((&(((*(sym_2315)))._f588)), (&(((*(sym_2315)))._f587)), sym_2318, sym_240());
    return;
}

void sym_173(S_57* sym_2319, S_51* sym_2320, uint32_t sym_2321) {
    uint32_t sym_2322 = sym_299((&(((*(sym_2319)))._f588)), sym_2321);
    uint32_t sym_2323 = sym_275((&(((*(sym_2319)))._f587)), ((uint16_t)8));
    uint32_t sym_2324 = sym_277((&(((*(sym_2319)))._f587)), sym_2323);
    sym_304((&(((*(sym_2319)))._f588)), (&(((*(sym_2319)))._f587)), sym_2322, sym_2324);
    return;
}

void sym_174(S_57* sym_2325, S_51* sym_2326, uint32_t sym_2327) {
    uint32_t sym_2328 = sym_299((&(((*(sym_2325)))._f588)), sym_2327);
    uint32_t sym_2329 = sym_95(sym_2326, sym_2327);
    uint32_t sym_2330 = sym_301((&(((*(sym_2325)))._f588)), sym_2329);
    if ((sym_2330 != sym_294())) {
        sym_303((&(((*(sym_2325)))._f588)), (&(((*(sym_2325)))._f587)), sym_2328, sym_2330);
    }
    return;
}

void sym_175(S_57* sym_2331, S_51* sym_2332, uint32_t sym_2333) {
    uint32_t sym_2334 = sym_299((&(((*(sym_2331)))._f588)), sym_2333);
    uint32_t sym_2335 = sym_95(sym_2332, sym_2333);
    uint32_t sym_2336 = sym_301((&(((*(sym_2331)))._f588)), sym_2335);
    if ((sym_2336 != sym_294())) {
        sym_304((&(((*(sym_2331)))._f588)), (&(((*(sym_2331)))._f587)), sym_2336, sym_240());
    }
    sym_304((&(((*(sym_2331)))._f588)), (&(((*(sym_2331)))._f587)), sym_2334, sym_240());
    return;
}

void sym_176(S_57* sym_2337, S_51* sym_2338, uint32_t sym_2339) {
    uint32_t sym_2340 = sym_299((&(((*(sym_2337)))._f588)), sym_2339);
    uint32_t sym_2341 = sym_95(sym_2338, sym_2339);
    uint32_t sym_2342 = sym_301((&(((*(sym_2337)))._f588)), sym_2341);
    if ((sym_2342 != sym_294())) {
        uint32_t sym_2343 = sym_307((&(((*(sym_2337)))._f588)), (&(((*(sym_2337)))._f587)), sym_2342);
        if ((sym_260((&(((*(sym_2337)))._f587)), sym_2343) == (sym_234())._f639)) {
            sym_304((&(((*(sym_2337)))._f588)), (&(((*(sym_2337)))._f587)), sym_2340, sym_241());
            uint32_t sym_2344 = sym_227(sym_2337, sym_2341);
            uint32_t sym_2345 = sym_277((&(((*(sym_2337)))._f587)), sym_2344);
            sym_226(sym_2337, sym_2339, sym_2345);
            return;
        }
        uint32_t sym_2346 = sym_277((&(((*(sym_2337)))._f587)), sym_2343);
        sym_304((&(((*(sym_2337)))._f588)), (&(((*(sym_2337)))._f587)), sym_2340, sym_2346);
    }
    return;
}

void sym_177(S_57* sym_2347, S_51* sym_2348, uint32_t sym_2349) {
    uint32_t sym_2350 = sym_299((&(((*(sym_2347)))._f588)), sym_2349);
    uint32_t sym_2351 = sym_95(sym_2348, sym_2349);
    uint32_t sym_2352 = sym_301((&(((*(sym_2347)))._f588)), sym_2351);
    if ((sym_2352 != sym_294())) {
        uint32_t sym_2353 = sym_307((&(((*(sym_2347)))._f588)), (&(((*(sym_2347)))._f587)), sym_2352);
        if ((sym_260((&(((*(sym_2347)))._f587)), sym_2353) == (sym_234())._f566)) {
            uint32_t sym_2354 = sym_261((&(((*(sym_2347)))._f587)), sym_2353);
            sym_304((&(((*(sym_2347)))._f588)), (&(((*(sym_2347)))._f587)), sym_2350, sym_2354);
        }
    }
    return;
}

void sym_178(S_57* sym_2355, S_51* sym_2356, uint32_t sym_2357) {
    uint32_t sym_2358 = sym_299((&(((*(sym_2355)))._f588)), sym_2357);
    sym_304((&(((*(sym_2355)))._f588)), (&(((*(sym_2355)))._f587)), sym_2358, sym_242());
    return;
}

void sym_179(S_57* sym_2359, S_51* sym_2360, uint32_t sym_2361) {
    uint32_t sym_2362 = sym_299((&(((*(sym_2359)))._f588)), sym_2361);
    uint32_t sym_2363 = sym_95(sym_2360, sym_2361);
    uint32_t sym_2364 = sym_96(sym_2360, sym_2361);
    uint32_t sym_2365 = sym_97(sym_2360, sym_2361);
    uint32_t sym_2366 = sym_301((&(((*(sym_2359)))._f588)), sym_2363);
    if ((sym_2366 != sym_294())) {
        sym_304((&(((*(sym_2359)))._f588)), (&(((*(sym_2359)))._f587)), sym_2366, sym_240());
    }
    uint32_t sym_2367 = sym_301((&(((*(sym_2359)))._f588)), sym_2364);
    if ((sym_2367 != sym_294())) {
        sym_303((&(((*(sym_2359)))._f588)), (&(((*(sym_2359)))._f587)), sym_2362, sym_2367);
    }
    if ((sym_2365 != sym_85())) {
        uint32_t sym_2368 = sym_301((&(((*(sym_2359)))._f588)), sym_2365);
        if ((sym_2368 != sym_294())) {
            sym_303((&(((*(sym_2359)))._f588)), (&(((*(sym_2359)))._f587)), sym_2362, sym_2368);
        }
    } else {
        sym_304((&(((*(sym_2359)))._f588)), (&(((*(sym_2359)))._f587)), sym_2362, sym_242());
    }
    return;
}

void sym_180(S_57* sym_2369, S_51* sym_2370, uint32_t sym_2371) {
    uint32_t sym_2372 = sym_299((&(((*(sym_2369)))._f588)), sym_2371);
    uint32_t sym_2373 = sym_95(sym_2370, sym_2371);
    uint32_t sym_2374 = sym_301((&(((*(sym_2369)))._f588)), sym_2373);
    if ((sym_2374 != sym_294())) {
        sym_304((&(((*(sym_2369)))._f588)), (&(((*(sym_2369)))._f587)), sym_2374, sym_240());
    }
    sym_304((&(((*(sym_2369)))._f588)), (&(((*(sym_2369)))._f587)), sym_2372, sym_242());
    return;
}

void sym_183(S_57* sym_2375, S_51* sym_2376, uint32_t sym_2377) {
    uint32_t sym_2378 = sym_299((&(((*(sym_2375)))._f588)), sym_2377);
    uint32_t sym_2379 = sym_96(sym_2376, sym_2377);
    uint32_t sym_2380 = sym_227(sym_2375, sym_2379);
    if ((sym_2380 != sym_236())) {
        sym_304((&(((*(sym_2375)))._f588)), (&(((*(sym_2375)))._f587)), sym_2378, sym_2380);
    }
    return;
}

void sym_184(S_57* sym_2381, S_51* sym_2382, uint32_t sym_2383) {
    S_55* sym_2384 = (&(((*(sym_2381)))._f588));
    S_54* sym_2385 = (&(((*(sym_2381)))._f587));
    uint32_t sym_2386 = sym_299(sym_2384, sym_2383);
    uint32_t sym_2387 = sym_95(sym_2382, sym_2383);
    uint32_t sym_2388 = sym_96(sym_2382, sym_2383);
    S_430 sym_2389 = sym_217(sym_2381, sym_2387);
    S_430 sym_2390 = sym_217(sym_2381, sym_2388);
    S_45 sym_2391 = sym_390();
    uint64_t sym_2392 = 0;
    while (((sym_2392 < (sym_2389)._f567) && (sym_2392 < (sym_2390)._f567))) {
        uint32_t sym_2393 = (*(((sym_2389)._f566 + sym_2392)));
        uint32_t sym_2394 = (*(((sym_2390)._f566 + sym_2392)));
        uint32_t sym_2395 = sym_95(sym_2382, sym_2393);
        uint32_t sym_2396 = sym_227(sym_2381, sym_2394);
        if ((sym_2396 == sym_236())) {
            uint32_t sym_2397 = sym_301(sym_2384, sym_2394);
            if ((sym_2397 != sym_294())) {
                sym_2396 = sym_307(sym_2384, sym_2385, sym_2397);
            }
        }
        sym_394((&(sym_2391)), sym_2395);
        sym_394((&(sym_2391)), sym_2396);
        sym_2392 = (sym_2392 + 1);
    }
    uint32_t* sym_2398 = ((uint32_t*)0);
    if (((sym_2391)._f567 > 0)) {
        sym_2398 = sym_404((&(sym_2391)), 0);
    }
    S_430 sym_2399 = ((S_430){._f566 = sym_2398, ._f567 = (sym_2391)._f567});
    uint32_t sym_2400 = sym_281(sym_2385, sym_2399);
    sym_392((&(sym_2391)));
    sym_226(sym_2381, sym_2383, sym_2400);
    sym_304(sym_2384, sym_2385, sym_2386, sym_241());
    return;
}

void sym_185(S_57* sym_2401, S_51* sym_2402, uint32_t sym_2403) {
    S_55* sym_2404 = (&(((*(sym_2401)))._f588));
    S_54* sym_2405 = (&(((*(sym_2401)))._f587));
    uint32_t sym_2406 = sym_299(sym_2404, sym_2403);
    uint32_t sym_2407 = sym_95(sym_2402, sym_2403);
    uint32_t sym_2408 = sym_96(sym_2402, sym_2403);
    S_430 sym_2409 = sym_217(sym_2401, sym_2407);
    S_430 sym_2410 = sym_217(sym_2401, sym_2408);
    S_45 sym_2411 = sym_390();
    uint64_t sym_2412 = 0;
    while (((sym_2412 < (sym_2409)._f567) && (sym_2412 < (sym_2410)._f567))) {
        uint32_t sym_2413 = (*(((sym_2409)._f566 + sym_2412)));
        uint32_t sym_2414 = (*(((sym_2410)._f566 + sym_2412)));
        uint32_t sym_2415 = sym_95(sym_2402, sym_2413);
        uint32_t sym_2416 = sym_301(sym_2404, sym_2414);
        if ((sym_2416 == sym_294())) {
            sym_2416 = sym_299(sym_2404, sym_2414);
        }
        uint32_t sym_2417 = sym_273(sym_2405, sym_2416);
        sym_394((&(sym_2411)), sym_2415);
        sym_394((&(sym_2411)), sym_2417);
        sym_2412 = (sym_2412 + 1);
    }
    uint32_t* sym_2418 = ((uint32_t*)0);
    if (((sym_2411)._f567 > 0)) {
        sym_2418 = sym_404((&(sym_2411)), 0);
    }
    S_430 sym_2419 = ((S_430){._f566 = sym_2418, ._f567 = (sym_2411)._f567});
    uint32_t sym_2420 = sym_281(sym_2405, sym_2419);
    sym_392((&(sym_2411)));
    sym_304(sym_2404, sym_2405, sym_2406, sym_2420);
    return;
}

void sym_186(S_57* sym_2421, S_51* sym_2422, uint32_t sym_2423) {
    S_55* sym_2424 = (&(((*(sym_2421)))._f588));
    S_54* sym_2425 = (&(((*(sym_2421)))._f587));
    uint32_t sym_2426 = sym_299(sym_2424, sym_2423);
    uint32_t sym_2427 = sym_95(sym_2422, sym_2423);
    uint32_t sym_2428 = sym_301(sym_2424, sym_2427);
    if ((sym_2428 != sym_294())) {
        uint32_t sym_2429 = sym_307(sym_2424, sym_2425, sym_2428);
        if ((sym_260(sym_2425, sym_2429) == (sym_234())._f566)) {
            sym_2429 = sym_261(sym_2425, sym_2429);
        }
        if ((sym_260(sym_2425, sym_2429) == (sym_234())._f650)) {
            uint32_t sym_2430 = sym_96(sym_2422, sym_2423);
            uint32_t sym_2431 = sym_95(sym_2422, sym_2430);
            S_430 sym_2432 = sym_290(sym_2425, sym_2429);
            uint64_t sym_2433 = ((sym_2432)._f567 / 2);
            bool sym_2434 = false;
            uint64_t sym_2435 = 0;
            while ((sym_2435 < sym_2433)) {
                uint32_t sym_2436 = (*(((sym_2432)._f566 + (sym_2435 * 2))));
                uint32_t sym_2437 = (*((((sym_2432)._f566 + (sym_2435 * 2)) + 1)));
                if ((sym_2436 == sym_2431)) {
                    sym_2434 = true;
                    if ((sym_260(sym_2425, sym_2437) == (sym_234())._f645)) {
                        uint32_t sym_2438 = sym_261(sym_2425, sym_2437);
                        sym_303(sym_2424, sym_2425, sym_2426, sym_2438);
                    } else {
                        sym_304(sym_2424, sym_2425, sym_2426, sym_2437);
                    }
                    uint32_t sym_2439 = sym_307(sym_2424, sym_2425, sym_2426);
                    print(((uint8_t*)"[MEMBER MATCH] field="));
                    print_u32(sym_2436);
                    print(((uint8_t*)" ty="));
                    print_u32(sym_2439);
                    print(((uint8_t*)" kind="));
                    print_u32(((uint32_t)sym_260(sym_2425, sym_2439)));
                    print(((uint8_t*)"\n"));
                    return;
                }
                sym_2435 = (sym_2435 + 1);
            }
            if ((!(sym_2434))) {
                print(((uint8_t*)"[MEMBER NOT FOUND!] target_name="));
                print_u32(sym_2431);
                print(((uint8_t*)" struct_ty="));
                print_u32(sym_2429);
                print(((uint8_t*)"\n"));
            }
        } else {
            print(((uint8_t*)"[MEMBER LHS NOT STRUCT!] lhs_ty="));
            print_u32(sym_2429);
            print(((uint8_t*)" kind="));
            print_u32(((uint32_t)sym_260(sym_2425, sym_2429)));
            print(((uint8_t*)"\n"));
        }
    }
    return;
}

void sym_182(S_57* sym_2442, S_51* sym_2443, uint32_t sym_2444) {
    uint32_t sym_2445 = sym_299((&(((*(sym_2442)))._f588)), sym_2444);
    uint32_t sym_2446 = sym_95(sym_2443, sym_2444);
    uint32_t sym_2447 = sym_96(sym_2443, sym_2444);
    uint32_t sym_2448 = sym_301((&(((*(sym_2442)))._f588)), sym_2446);
    uint32_t sym_2449 = sym_301((&(((*(sym_2442)))._f588)), sym_2447);
    if ((sym_2448 != sym_294())) {
        sym_304((&(((*(sym_2442)))._f588)), (&(((*(sym_2442)))._f587)), sym_2448, sym_240());
    }
    if ((sym_2449 != sym_294())) {
        sym_304((&(((*(sym_2442)))._f588)), (&(((*(sym_2442)))._f587)), sym_2449, sym_240());
    }
    sym_304((&(((*(sym_2442)))._f588)), (&(((*(sym_2442)))._f587)), sym_2445, sym_240());
    return;
}

void sym_181(S_57* sym_2450, S_51* sym_2451, uint32_t sym_2452) {
    uint32_t sym_2453 = sym_299((&(((*(sym_2450)))._f588)), sym_2452);
    uint32_t sym_2454 = sym_95(sym_2451, sym_2452);
    uint32_t sym_2455 = sym_96(sym_2451, sym_2452);
    uint32_t sym_2456 = sym_301((&(((*(sym_2450)))._f588)), sym_2454);
    uint32_t sym_2457 = sym_301((&(((*(sym_2450)))._f588)), sym_2455);
    if (((sym_2456 != sym_294()) && (sym_2457 != sym_294()))) {
        sym_303((&(((*(sym_2450)))._f588)), (&(((*(sym_2450)))._f587)), sym_2456, sym_2457);
    }
    sym_304((&(((*(sym_2450)))._f588)), (&(((*(sym_2450)))._f587)), sym_2453, sym_240());
    return;
}

void sym_320(S_59* sym_2458, uint32_t sym_2459) {
    sym_330((&(((*(sym_2458)))._f542)), sym_2459);
    return;
}

void sym_330(S_58* sym_2460, uint32_t sym_2461) {
    ((*(sym_2460)))._f627 = sym_2461;
    ((*(sym_2460)))._f628 = true;
    return;
}

void sym_321(S_59* sym_2462, S_14* sym_2463, uint32_t sym_2464) {
    S_45 sym_2465 = sym_390();
    sym_576((&(((*(sym_2462)))._f579)), (&(((*(sym_2462)))._f542)), sym_2463, sym_2464, (&(sym_2465)));
    sym_392((&(sym_2465)));
    return;
}

void sym_576(S_57* sym_2466, S_58* sym_2467, S_14* sym_2468, uint32_t sym_2469, S_45* sym_2470) {
    if (sym_572(sym_2470, sym_2469)) {
        return;
    }
    sym_394(sym_2470, sym_2469);
    uint32_t sym_2471 = sym_538((&(((*(sym_2466)))._f594)), sym_2469);
    if ((sym_2471 == 0)) {
        return;
    }
    uint32_t sym_2472 = (sym_2471 - 1);
    S_351 sym_2473 = sym_87();
    if ((sym_94((&(((*(sym_2466)))._f583)), sym_2472) != (sym_2473)._f890)) {
        return;
    }
    uint32_t sym_2474 = sym_97((&(((*(sym_2466)))._f583)), sym_2472);
    if ((sym_94((&(((*(sym_2466)))._f583)), sym_2474) != (sym_2473)._f888)) {
        return;
    }
    uint32_t sym_2475 = sym_571(sym_2466, sym_2474);
    uint32_t sym_2476 = sym_285((&(((*(sym_2466)))._f587)), sym_2475);
    S_23 sym_2477 = sym_570((&(((*(sym_2466)))._f587)), sym_2468, sym_2476);
    uint32_t sym_2478 = sym_97((&(((*(sym_2466)))._f583)), sym_2474);
    bool sym_2479 = ((sym_2478 == sym_85()) || sym_287((&(((*(sym_2466)))._f587)), sym_2475));
    S_23 sym_2480 = sym_49();
    if (sym_2479) {
        uint32_t sym_2481 = sym_95((&(((*(sym_2466)))._f583)), sym_2472);
        uint32_t sym_2482 = sym_95((&(((*(sym_2466)))._f583)), sym_2481);
        sym_2480 = sym_212((&(((*(sym_2466)))._f586)), sym_2482);
    } else {
        S_568 sym_2483 = sym_56(sym_2468);
        sym_57((&(sym_2483)), sym_50(((uint8_t*)"sym_")));
        sym_60((&(sym_2483)), sym_2469);
        sym_2480 = sym_58((&(sym_2483)));
    }
    S_430 sym_2484 = sym_288((&(((*(sym_2466)))._f587)), sym_2475);
    S_23 sym_2485 = sym_50(((uint8_t*)"void"));
    if (((sym_2484)._f567 > 0)) {
        S_568 sym_2486 = sym_56(sym_2468);
        uint64_t sym_2487 = 0;
        while ((sym_2487 < (sym_2484)._f567)) {
            uint32_t sym_2488 = (*(((sym_2484)._f566 + sym_2487)));
            S_23 sym_2489 = sym_570((&(((*(sym_2466)))._f587)), sym_2468, sym_2488);
            sym_57((&(sym_2486)), sym_2489);
            sym_57((&(sym_2486)), sym_50(((uint8_t*)" ")));
            if (sym_2479) {
                sym_57((&(sym_2486)), sym_50(((uint8_t*)"_a")));
                sym_60((&(sym_2486)), ((uint32_t)sym_2487));
            } else {
                uint32_t sym_2490 = sym_95((&(((*(sym_2466)))._f583)), sym_2474);
                if ((sym_2490 != sym_85())) {
                    S_430 sym_2491 = sym_217(sym_2466, sym_2490);
                    uint32_t sym_2492 = (*(((sym_2491)._f566 + sym_2487)));
                    uint32_t sym_2493 = sym_95((&(((*(sym_2466)))._f583)), sym_2492);
                    uint32_t sym_2494 = sym_225(sym_2466, sym_2493);
                    sym_57((&(sym_2486)), sym_50(((uint8_t*)"sym_")));
                    sym_60((&(sym_2486)), sym_2494);
                }
            }
            if (((sym_2487 + 1) < (sym_2484)._f567)) {
                sym_57((&(sym_2486)), sym_50(((uint8_t*)", ")));
            }
            sym_2487 = (sym_2487 + 1);
        }
        sym_2485 = sym_58((&(sym_2486)));
    }
    if (sym_2479) {
        return;
    }
    S_45 sym_2495 = sym_390();
    S_45 sym_2496 = sym_390();
    sym_332(sym_2467, sym_2480, sym_2477, sym_2485);
    S_23 sym_2497 = sym_575(sym_2466, sym_2467, sym_2468, sym_2478, sym_2470, (&(sym_2495)), (&(sym_2496)));
    if ((sym_260((&(((*(sym_2466)))._f587)), sym_2476) == (sym_234())._f547)) {
        if (((sym_2497)._f567 > 0)) {
            sym_335(sym_2467, sym_2497);
        }
        sym_336(sym_2467, sym_49(), false);
    } else {
        if (((sym_2497)._f567 > 0)) {
            sym_336(sym_2467, sym_2497, true);
        }
    }
    sym_333(sym_2467);
    uint64_t sym_2498 = 0;
    while ((sym_2498 < (sym_2495)._f567)) {
        uint32_t sym_2499 = sym_396((&(sym_2495)), sym_2498);
        sym_576(sym_2466, sym_2467, sym_2468, sym_2499, sym_2470);
        sym_2498 = (sym_2498 + 1);
    }
    S_45 sym_2500 = sym_596((&(((*(sym_2466)))._f587)), (&(sym_2496)));
    uint64_t sym_2501 = 0;
    while ((sym_2501 < (sym_2500)._f567)) {
        uint32_t sym_2502 = sym_396((&(sym_2500)), sym_2501);
        if ((sym_260((&(((*(sym_2466)))._f587)), sym_2502) == (sym_234())._f650)) {
            S_23 sym_2503 = sym_293((&(((*(sym_2466)))._f587)), sym_2468, sym_2502);
            S_568 sym_2504 = sym_56(sym_2468);
            S_430 sym_2505 = sym_290((&(((*(sym_2466)))._f587)), sym_2502);
            uint64_t sym_2506 = ((sym_2505)._f567 / 2);
            uint64_t sym_2507 = 0;
            while ((sym_2507 < sym_2506)) {
                uint32_t sym_2508 = (*(((sym_2505)._f566 + (sym_2507 * 2))));
                uint32_t sym_2509 = (*((((sym_2505)._f566 + (sym_2507 * 2)) + 1)));
                S_23 sym_2510 = sym_570((&(((*(sym_2466)))._f587)), sym_2468, sym_2509);
                sym_57((&(sym_2504)), sym_50(((uint8_t*)"    ")));
                sym_57((&(sym_2504)), sym_2510);
                sym_57((&(sym_2504)), sym_50(((uint8_t*)" _f")));
                sym_60((&(sym_2504)), sym_2508);
                sym_57((&(sym_2504)), sym_50(((uint8_t*)";\n")));
                sym_2507 = (sym_2507 + 1);
            }
            sym_343(sym_2467, sym_2502, sym_2503, sym_58((&(sym_2504))));
        }
        sym_2501 = (sym_2501 + 1);
    }
    sym_392((&(sym_2500)));
    sym_392((&(sym_2496)));
    return;
}

bool sym_572(S_45* sym_2511, uint32_t sym_2512) {
    uint64_t sym_2513 = 0;
    while ((sym_2513 < ((*(sym_2511)))._f567)) {
        if ((sym_396(sym_2511, sym_2513) == sym_2512)) {
            return true;
        }
        sym_2513 = (sym_2513 + 1);
    }
    return false;
}

uint32_t sym_571(S_57* sym_2514, uint32_t sym_2515) {
    if ((sym_2515 == sym_85())) {
        return sym_236();
    }
    uint32_t sym_2516 = sym_301((&(((*(sym_2514)))._f588)), sym_2515);
    if ((sym_2516 == sym_294())) {
        return sym_236();
    }
    return sym_307((&(((*(sym_2514)))._f588)), (&(((*(sym_2514)))._f587)), sym_2516);
}

S_23 sym_570(S_54* sym_2517, S_14* sym_2518, uint32_t sym_2519) {
    if ((sym_2519 == sym_236())) {
        return sym_50(((uint8_t*)"void"));
    }
    S_112 sym_2520 = sym_234();
    uint8_t sym_2521 = sym_260(sym_2517, sym_2519);
    uint32_t sym_2522 = sym_261(sym_2517, sym_2519);
    if ((sym_2521 == (sym_2520)._f547)) {
        return sym_50(((uint8_t*)"void"));
    }
    if ((sym_2521 == (sym_2520)._f604)) {
        return sym_50(((uint8_t*)"bool"));
    }
    if ((sym_2521 == (sym_2520)._f646)) {
        if ((sym_2522 == 8)) {
            return sym_50(((uint8_t*)"int8_t"));
        }
        if ((sym_2522 == 16)) {
            return sym_50(((uint8_t*)"int16_t"));
        }
        if ((sym_2522 == 32)) {
            return sym_50(((uint8_t*)"int32_t"));
        }
        if ((sym_2522 == 64)) {
            return sym_50(((uint8_t*)"int64_t"));
        }
        if ((sym_2522 == 128)) {
            return sym_50(((uint8_t*)"__int128"));
        }
        return sym_50(((uint8_t*)"int32_t"));
    }
    if ((sym_2521 == (sym_2520)._f647)) {
        if ((sym_2522 == 8)) {
            return sym_50(((uint8_t*)"uint8_t"));
        }
        if ((sym_2522 == 16)) {
            return sym_50(((uint8_t*)"uint16_t"));
        }
        if ((sym_2522 == 32)) {
            return sym_50(((uint8_t*)"uint32_t"));
        }
        if ((sym_2522 == 64)) {
            return sym_50(((uint8_t*)"uint64_t"));
        }
        if ((sym_2522 == 128)) {
            return sym_50(((uint8_t*)"unsigned __int128"));
        }
        return sym_50(((uint8_t*)"uint32_t"));
    }
    if ((sym_2521 == (sym_2520)._f648)) {
        if ((sym_2522 == 32)) {
            return sym_50(((uint8_t*)"float"));
        }
        if ((sym_2522 == 64)) {
            return sym_50(((uint8_t*)"double"));
        }
        return sym_50(((uint8_t*)"double"));
    }
    if ((sym_2521 == (sym_2520)._f637)) {
        return sym_50(((uint8_t*)"int32_t"));
    }
    if ((sym_2521 == (sym_2520)._f638)) {
        return sym_50(((uint8_t*)"double"));
    }
    if ((sym_2521 == (sym_2520)._f566)) {
        uint32_t sym_2523 = sym_261(sym_2517, sym_2519);
        S_23 sym_2524 = sym_570(sym_2517, sym_2518, sym_2523);
        S_568 sym_2525 = sym_56(sym_2518);
        sym_57((&(sym_2525)), sym_2524);
        sym_57((&(sym_2525)), sym_50(((uint8_t*)"*")));
        return sym_58((&(sym_2525)));
    }
    if ((sym_2521 == (sym_2520)._f650)) {
        return sym_293(sym_2517, sym_2518, sym_2519);
    }
    if ((((sym_2521 == (sym_2520)._f645) || (sym_2521 == (sym_2520)._f636)) || (sym_2521 == (sym_2520)._f641))) {
        return sym_50(((uint8_t*)"uint64_t"));
    }
    return sym_50(((uint8_t*)"void"));
}

S_568 sym_56(S_14* sym_2526) {
    return ((S_568){._f564 = sym_515(sym_2526)});
}

S_567 sym_515(S_14* sym_2527) {
    uint64_t sym_2528 = 4;
    uint64_t sym_2529 = (sym_2528 * sym_577());
    uint8_t* sym_2530 = ((uint8_t*)sym_70(sym_2527, sym_2529));
    return ((S_567){._f533 = sym_2527, ._f566 = sym_2530, ._f567 = 0, ._f554 = sym_2528});
}

void sym_57(S_568* sym_2531, S_23 sym_2532) {
    uint64_t sym_2533 = 0;
    while ((sym_2533 < (sym_2532)._f567)) {
        sym_518((&(((*(sym_2531)))._f564)), (*(((sym_2532)._f566 + sym_2533))));
        sym_2533 = (sym_2533 + 1);
    }
    return;
}

void sym_518(S_567* sym_2534, uint8_t sym_2535) {
    sym_517(sym_2534);
    (*((((*(sym_2534)))._f566 + ((*(sym_2534)))._f567))) = sym_2535;
    ((*(sym_2534)))._f567 = (((*(sym_2534)))._f567 + 1);
    return;
}

void sym_517(S_567* sym_2536) {
    if ((((*(sym_2536)))._f567 == ((*(sym_2536)))._f554)) {
        uint64_t sym_2537 = (((*(sym_2536)))._f554 * 2);
        uint64_t sym_2538 = (((*(sym_2536)))._f554 * sym_577());
        uint64_t sym_2539 = (sym_2537 * sym_577());
        uint8_t* sym_2540 = ((uint8_t*)sym_72(((*(sym_2536)))._f533, ((uint8_t*)((*(sym_2536)))._f566), sym_2538, sym_2539));
        ((*(sym_2536)))._f566 = sym_2540;
        ((*(sym_2536)))._f554 = sym_2537;
    }
    return;
}

uint8_t* sym_72(S_14* sym_2541, uint8_t* sym_2542, uint64_t sym_2543, uint64_t sym_2544) {
    if ((sym_2544 <= sym_2543)) {
        return sym_2542;
    }
    uint64_t sym_2545 = sym_67(sym_2543);
    uint64_t sym_2546 = sym_67(sym_2544);
    S_12* sym_2547 = ((*(sym_2541)))._f104;
    if (((sym_2547 != ((S_12*)0)) && (((*(sym_2547)))._f553 >= sym_2545))) {
        uint8_t* sym_2548 = (((*(sym_2547)))._f552 + (((*(sym_2547)))._f553 - sym_2545));
        if ((sym_2542 == sym_2548)) {
            uint64_t sym_2549 = (sym_2546 - sym_2545);
            if (((((*(sym_2547)))._f553 + sym_2549) <= ((*(sym_2547)))._f554)) {
                ((*(sym_2547)))._f553 = (((*(sym_2547)))._f553 + sym_2549);
                return sym_2542;
            }
        }
    }
    uint8_t* sym_2550 = sym_70(sym_2541, sym_2544);
    if (((sym_2542 != ((uint8_t*)0)) && (sym_2543 > 0))) {
        mem_copy(sym_2550, sym_2542, sym_2543);
    }
    return sym_2550;
}

S_23 sym_58(S_568* sym_2554) {
    return ((S_23){._f566 = (((*(sym_2554)))._f564)._f566, ._f567 = (((*(sym_2554)))._f564)._f567});
}

S_23 sym_293(S_54* sym_2555, S_14* sym_2556, uint32_t sym_2557) {
    S_568 sym_2558 = sym_56(sym_2556);
    sym_57((&(sym_2558)), sym_50(((uint8_t*)"S_")));
    sym_60((&(sym_2558)), sym_2557);
    return sym_58((&(sym_2558)));
}

void sym_60(S_568* sym_2559, uint32_t sym_2560) {
    if ((sym_2560 == 0)) {
        sym_59(sym_2559, (*(((uint8_t*)"0"))));
        return;
    }
    uint32_t sym_2561 = sym_2560;
    uint32_t sym_2562 = 1;
    while (((((sym_2561 / sym_2562) >= 10) && (sym_2562 <= 100000000)) || false)) {
        sym_2562 = (sym_2562 * 10);
    }
    while (((sym_2562 > 0) || false)) {
        uint32_t sym_2563 = (sym_2561 / sym_2562);
        sym_59(sym_2559, ((uint8_t)(sym_2563 + 48)));
        sym_2561 = (sym_2561 - (sym_2563 * sym_2562));
        sym_2562 = (sym_2562 / 10);
    }
    return;
}

void sym_59(S_568* sym_2564, uint8_t sym_2565) {
    sym_518((&(((*(sym_2564)))._f564)), sym_2565);
    return;
}

bool sym_287(S_54* sym_2566, uint32_t sym_2567) {
    uint64_t sym_2568 = ((uint64_t)sym_261(sym_2566, sym_2567));
    uint32_t sym_2569 = sym_396((&((((*(sym_2566)))._f612)._f609)), sym_2568);
    return ((sym_2569 & 65535) != 0);
}

void sym_332(S_58* sym_2570, S_23 sym_2571, S_23 sym_2572, S_23 sym_2573) {
    sym_331(sym_2570, sym_2571, sym_2572, sym_2573);
    sym_322((&(((*(sym_2570)))._f625)), sym_2572);
    sym_323((&(((*(sym_2570)))._f625)), ((uint8_t)32));
    sym_322((&(((*(sym_2570)))._f625)), sym_2571);
    sym_323((&(((*(sym_2570)))._f625)), ((uint8_t)40));
    sym_322((&(((*(sym_2570)))._f625)), sym_2573);
    sym_322((&(((*(sym_2570)))._f625)), sym_50(((uint8_t*)") {\n")));
    ((*(sym_2570)))._f629 = (((*(sym_2570)))._f629 + 1);
    return;
}

void sym_331(S_58* sym_2574, S_23 sym_2575, S_23 sym_2576, S_23 sym_2577) {
    sym_322((&(((*(sym_2574)))._f624)), sym_2576);
    sym_323((&(((*(sym_2574)))._f624)), ((uint8_t)32));
    sym_322((&(((*(sym_2574)))._f624)), sym_2575);
    sym_323((&(((*(sym_2574)))._f624)), ((uint8_t)40));
    sym_322((&(((*(sym_2574)))._f624)), sym_2577);
    sym_322((&(((*(sym_2574)))._f624)), sym_50(((uint8_t*)");\n")));
    return;
}

void sym_322(S_42* sym_2578, S_23 sym_2579) {
    uint64_t sym_2580 = 0;
    while ((sym_2580 < (sym_2579)._f567)) {
        sym_352(sym_2578, (*(((sym_2579)._f566 + sym_2580))));
        sym_2580 = (sym_2580 + 1);
    }
    return;
}

void sym_323(S_42* sym_2581, uint8_t sym_2582) {
    sym_352(sym_2581, sym_2582);
    return;
}

S_23 sym_575(S_57* sym_2583, S_58* sym_2584, S_14* sym_2585, uint32_t sym_2586, S_45* sym_2587, S_45* sym_2588, S_45* sym_2589) {
    S_351 sym_2590 = sym_87();
    uint8_t sym_2591 = sym_94((&(((*(sym_2583)))._f583)), sym_2586);
    if ((sym_2591 == (sym_2590)._f670)) {
        uint32_t sym_2592 = sym_95((&(((*(sym_2583)))._f583)), sym_2586);
        S_568 sym_2593 = sym_56(sym_2585);
        sym_60((&(sym_2593)), sym_2592);
        return sym_58((&(sym_2593)));
    }
    if ((sym_2591 == (sym_2590)._f673)) {
        uint32_t sym_2594 = sym_571(sym_2583, sym_2586);
        uint32_t sym_2595 = sym_225(sym_2583, sym_2586);
        if ((sym_260((&(((*(sym_2583)))._f587)), sym_2594) == (sym_234())._f649)) {
            if ((sym_2595 != sym_532())) {
                if ((!(sym_572(sym_2588, sym_2595)))) {
                    sym_394(sym_2588, sym_2595);
                }
            }
            if (sym_287((&(((*(sym_2583)))._f587)), sym_2594)) {
                uint32_t sym_2596 = sym_95((&(((*(sym_2583)))._f583)), sym_2586);
                return sym_212((&(((*(sym_2583)))._f586)), sym_2596);
            }
            S_568 sym_2597 = sym_56(sym_2585);
            sym_57((&(sym_2597)), sym_50(((uint8_t*)"sym_")));
            sym_60((&(sym_2597)), sym_2595);
            return sym_58((&(sym_2597)));
        }
        if ((sym_2595 != sym_532())) {
            S_568 sym_2598 = sym_56(sym_2585);
            sym_57((&(sym_2598)), sym_50(((uint8_t*)"sym_")));
            sym_60((&(sym_2598)), sym_2595);
            return sym_58((&(sym_2598)));
        }
        uint32_t sym_2599 = sym_95((&(((*(sym_2583)))._f583)), sym_2586);
        return sym_212((&(((*(sym_2583)))._f586)), sym_2599);
    }
    if ((sym_2591 == (sym_2590)._f891)) {
        uint32_t sym_2600 = sym_95((&(((*(sym_2583)))._f583)), sym_2586);
        uint32_t sym_2601 = sym_97((&(((*(sym_2583)))._f583)), sym_2586);
        uint32_t sym_2602 = sym_225(sym_2583, sym_2600);
        uint32_t sym_2603 = sym_571(sym_2583, sym_2600);
        S_23 sym_2604 = sym_570((&(((*(sym_2583)))._f587)), sym_2585, sym_2603);
        S_568 sym_2605 = sym_56(sym_2585);
        sym_57((&(sym_2605)), sym_50(((uint8_t*)"sym_")));
        sym_60((&(sym_2605)), sym_2602);
        S_23 sym_2606 = sym_58((&(sym_2605)));
        if (((sym_2603 == sym_236()) || (sym_260((&(((*(sym_2583)))._f587)), sym_2603) == (sym_234())._f636))) {
            print(((uint8_t*)"[LOWER VAR UNKNOWN!] sym="));
            print_u32(sym_2602);
            print(((uint8_t*)" hir="));
            print_u32(sym_2586);
            print(((uint8_t*)"\n"));
        }
        if ((sym_2601 != sym_85())) {
            S_23 sym_2607 = sym_575(sym_2583, sym_2584, sym_2585, sym_2601, sym_2587, sym_2588, sym_2589);
            sym_334(sym_2584, sym_2606, sym_2604, sym_2607, true);
        } else {
            sym_334(sym_2584, sym_2606, sym_2604, sym_49(), false);
        }
        sym_574(sym_2583, sym_2603, sym_2589);
        return sym_49();
    }
    if ((sym_2591 == (sym_2590)._f852)) {
        uint32_t sym_2608 = sym_95((&(((*(sym_2583)))._f583)), sym_2586);
        uint32_t sym_2609 = sym_96((&(((*(sym_2583)))._f583)), sym_2586);
        S_23 sym_2610 = sym_575(sym_2583, sym_2584, sym_2585, sym_2608, sym_2587, sym_2588, sym_2589);
        S_23 sym_2611 = sym_575(sym_2583, sym_2584, sym_2585, sym_2609, sym_2587, sym_2588, sym_2589);
        S_568 sym_2612 = sym_56(sym_2585);
        sym_57((&(sym_2612)), sym_2610);
        sym_57((&(sym_2612)), sym_50(((uint8_t*)" = ")));
        sym_57((&(sym_2612)), sym_2611);
        return sym_58((&(sym_2612)));
    }
    if (((((((((((((((((((sym_2591 == (sym_2590)._f863) || (sym_2591 == (sym_2590)._f864)) || (sym_2591 == (sym_2590)._f865)) || (sym_2591 == (sym_2590)._f866)) || (sym_2591 == (sym_2590)._f867)) || (sym_2591 == (sym_2590)._f868)) || (sym_2591 == (sym_2590)._f869)) || (sym_2591 == (sym_2590)._f870)) || (sym_2591 == (sym_2590)._f871)) || (sym_2591 == (sym_2590)._f872)) || (sym_2591 == (sym_2590)._f873)) || (sym_2591 == (sym_2590)._f876)) || (sym_2591 == (sym_2590)._f877)) || (sym_2591 == (sym_2590)._f878)) || (sym_2591 == (sym_2590)._f879)) || (sym_2591 == (sym_2590)._f880)) || (sym_2591 == (sym_2590)._f874)) || (sym_2591 == (sym_2590)._f875))) {
        uint32_t sym_2613 = sym_95((&(((*(sym_2583)))._f583)), sym_2586);
        uint32_t sym_2614 = sym_96((&(((*(sym_2583)))._f583)), sym_2586);
        S_23 sym_2615 = sym_575(sym_2583, sym_2584, sym_2585, sym_2613, sym_2587, sym_2588, sym_2589);
        S_23 sym_2616 = sym_575(sym_2583, sym_2584, sym_2585, sym_2614, sym_2587, sym_2588, sym_2589);
        S_23 sym_2617 = sym_573(sym_2590, sym_2591);
        S_568 sym_2618 = sym_56(sym_2585);
        sym_57((&(sym_2618)), sym_50(((uint8_t*)"(")));
        sym_57((&(sym_2618)), sym_2615);
        sym_57((&(sym_2618)), sym_2617);
        sym_57((&(sym_2618)), sym_2616);
        sym_57((&(sym_2618)), sym_50(((uint8_t*)")")));
        return sym_58((&(sym_2618)));
    }
    if ((sym_2591 == (sym_2590)._f889)) {
        uint32_t sym_2619 = sym_95((&(((*(sym_2583)))._f583)), sym_2586);
        uint32_t sym_2620 = sym_96((&(((*(sym_2583)))._f583)), sym_2586);
        uint32_t sym_2621 = sym_225(sym_2583, sym_2619);
        if ((sym_2621 != sym_532())) {
            if ((!(sym_572(sym_2588, sym_2621)))) {
                sym_394(sym_2588, sym_2621);
            }
        }
        S_23 sym_2622 = sym_575(sym_2583, sym_2584, sym_2585, sym_2619, sym_2587, sym_2588, sym_2589);
        S_568 sym_2623 = sym_56(sym_2585);
        sym_57((&(sym_2623)), sym_2622);
        sym_57((&(sym_2623)), sym_50(((uint8_t*)"(")));
        if ((sym_2620 != sym_85())) {
            S_430 sym_2624 = sym_217(sym_2583, sym_2620);
            uint64_t sym_2625 = 0;
            while ((sym_2625 < (sym_2624)._f567)) {
                uint32_t sym_2626 = (*(((sym_2624)._f566 + sym_2625)));
                S_23 sym_2627 = sym_575(sym_2583, sym_2584, sym_2585, sym_2626, sym_2587, sym_2588, sym_2589);
                sym_57((&(sym_2623)), sym_2627);
                if (((sym_2625 + 1) < (sym_2624)._f567)) {
                    sym_57((&(sym_2623)), sym_50(((uint8_t*)", ")));
                }
                sym_2625 = (sym_2625 + 1);
            }
        }
        sym_57((&(sym_2623)), sym_50(((uint8_t*)")")));
        return sym_58((&(sym_2623)));
    }
    if ((sym_2591 == (sym_2590)._f893)) {
        uint32_t sym_2628 = sym_95((&(((*(sym_2583)))._f583)), sym_2586);
        S_430 sym_2629 = sym_217(sym_2583, sym_2628);
        uint64_t sym_2630 = (sym_2629)._f567;
        S_23 sym_2631 = sym_49();
        uint64_t sym_2632 = 0;
        while ((sym_2632 < sym_2630)) {
            uint32_t sym_2633 = (*(((sym_2629)._f566 + sym_2632)));
            bool sym_2634 = ((sym_2632 + 1) == sym_2630);
            S_23 sym_2635 = sym_575(sym_2583, sym_2584, sym_2585, sym_2633, sym_2587, sym_2588, sym_2589);
            if ((!(sym_2634))) {
                sym_335(sym_2584, sym_2635);
                print(((uint8_t*)""));
            } else {
                sym_2631 = sym_2635;
                print(((uint8_t*)""));
            }
            sym_2632 = (sym_2632 + 1);
        }
        return sym_2631;
    }
    if ((sym_2591 == (sym_2590)._f672)) {
        uint32_t sym_2636 = sym_95((&(((*(sym_2583)))._f583)), sym_2586);
        return sym_212((&(((*(sym_2583)))._f586)), sym_2636);
    }
    if ((sym_2591 == (sym_2590)._f850)) {
        return sym_50(((uint8_t*)"true"));
    }
    if ((sym_2591 == (sym_2590)._f851)) {
        return sym_50(((uint8_t*)"false"));
    }
    if ((sym_2591 == (sym_2590)._f674)) {
        uint32_t sym_2637 = sym_95((&(((*(sym_2583)))._f583)), sym_2586);
        S_23 sym_2638 = sym_212((&(((*(sym_2583)))._f586)), sym_2637);
        S_568 sym_2639 = sym_56(sym_2585);
        sym_57((&(sym_2639)), sym_50(((uint8_t*)"((uint8_t*)")));
        sym_57((&(sym_2639)), sym_2638);
        sym_57((&(sym_2639)), sym_50(((uint8_t*)")")));
        return sym_58((&(sym_2639)));
    }
    if ((sym_2591 == (sym_2590)._f882)) {
        S_23 sym_2640 = sym_575(sym_2583, sym_2584, sym_2585, sym_95((&(((*(sym_2583)))._f583)), sym_2586), sym_2587, sym_2588, sym_2589);
        S_568 sym_2641 = sym_56(sym_2585);
        sym_57((&(sym_2641)), sym_50(((uint8_t*)"(-(")));
        sym_57((&(sym_2641)), sym_2640);
        sym_57((&(sym_2641)), sym_50(((uint8_t*)"))")));
        return sym_58((&(sym_2641)));
    }
    if ((sym_2591 == (sym_2590)._f883)) {
        S_23 sym_2642 = sym_575(sym_2583, sym_2584, sym_2585, sym_95((&(((*(sym_2583)))._f583)), sym_2586), sym_2587, sym_2588, sym_2589);
        S_568 sym_2643 = sym_56(sym_2585);
        sym_57((&(sym_2643)), sym_50(((uint8_t*)"(!(")));
        sym_57((&(sym_2643)), sym_2642);
        sym_57((&(sym_2643)), sym_50(((uint8_t*)"))")));
        return sym_58((&(sym_2643)));
    }
    if ((sym_2591 == (sym_2590)._f884)) {
        S_23 sym_2644 = sym_575(sym_2583, sym_2584, sym_2585, sym_95((&(((*(sym_2583)))._f583)), sym_2586), sym_2587, sym_2588, sym_2589);
        S_568 sym_2645 = sym_56(sym_2585);
        sym_57((&(sym_2645)), sym_50(((uint8_t*)"(~(")));
        sym_57((&(sym_2645)), sym_2644);
        sym_57((&(sym_2645)), sym_50(((uint8_t*)"))")));
        return sym_58((&(sym_2645)));
    }
    if ((sym_2591 == (sym_2590)._f886)) {
        S_23 sym_2646 = sym_575(sym_2583, sym_2584, sym_2585, sym_95((&(((*(sym_2583)))._f583)), sym_2586), sym_2587, sym_2588, sym_2589);
        S_568 sym_2647 = sym_56(sym_2585);
        sym_57((&(sym_2647)), sym_50(((uint8_t*)"(&(")));
        sym_57((&(sym_2647)), sym_2646);
        sym_57((&(sym_2647)), sym_50(((uint8_t*)"))")));
        return sym_58((&(sym_2647)));
    }
    if ((sym_2591 == (sym_2590)._f885)) {
        S_23 sym_2648 = sym_575(sym_2583, sym_2584, sym_2585, sym_95((&(((*(sym_2583)))._f583)), sym_2586), sym_2587, sym_2588, sym_2589);
        S_568 sym_2649 = sym_56(sym_2585);
        sym_57((&(sym_2649)), sym_50(((uint8_t*)"(*(")));
        sym_57((&(sym_2649)), sym_2648);
        sym_57((&(sym_2649)), sym_50(((uint8_t*)"))")));
        return sym_58((&(sym_2649)));
    }
    if ((sym_2591 == (sym_2590)._f896)) {
        sym_335(sym_2584, sym_50(((uint8_t*)"break")));
        return sym_49();
    }
    if ((sym_2591 == (sym_2590)._f897)) {
        sym_335(sym_2584, sym_50(((uint8_t*)"continue")));
        return sym_49();
    }
    if ((sym_2591 == (sym_2590)._f898)) {
        S_23 sym_2650 = sym_575(sym_2583, sym_2584, sym_2585, sym_95((&(((*(sym_2583)))._f583)), sym_2586), sym_2587, sym_2588, sym_2589);
        S_23 sym_2651 = sym_570((&(((*(sym_2583)))._f587)), sym_2585, sym_571(sym_2583, sym_2586));
        S_568 sym_2652 = sym_56(sym_2585);
        sym_57((&(sym_2652)), sym_50(((uint8_t*)"((")));
        sym_57((&(sym_2652)), sym_2651);
        sym_57((&(sym_2652)), sym_50(((uint8_t*)")")));
        sym_57((&(sym_2652)), sym_2650);
        sym_57((&(sym_2652)), sym_50(((uint8_t*)")")));
        return sym_58((&(sym_2652)));
    }
    if ((sym_2591 == (sym_2590)._f894)) {
        S_23 sym_2653 = sym_575(sym_2583, sym_2584, sym_2585, sym_95((&(((*(sym_2583)))._f583)), sym_2586), sym_2587, sym_2588, sym_2589);
        sym_338(sym_2584, sym_2653);
        S_23 sym_2654 = sym_575(sym_2583, sym_2584, sym_2585, sym_96((&(((*(sym_2583)))._f583)), sym_2586), sym_2587, sym_2588, sym_2589);
        if (((sym_2654)._f567 > 0)) {
            sym_335(sym_2584, sym_2654);
        }
        uint32_t sym_2655 = sym_97((&(((*(sym_2583)))._f583)), sym_2586);
        if ((sym_2655 != sym_85())) {
            sym_339(sym_2584);
            S_23 sym_2656 = sym_575(sym_2583, sym_2584, sym_2585, sym_2655, sym_2587, sym_2588, sym_2589);
            if (((sym_2656)._f567 > 0)) {
                sym_335(sym_2584, sym_2656);
            }
        }
        sym_341(sym_2584);
        return sym_49();
    }
    if ((sym_2591 == (sym_2590)._f895)) {
        S_23 sym_2657 = sym_575(sym_2583, sym_2584, sym_2585, sym_95((&(((*(sym_2583)))._f583)), sym_2586), sym_2587, sym_2588, sym_2589);
        sym_340(sym_2584, sym_2657);
        S_23 sym_2658 = sym_575(sym_2583, sym_2584, sym_2585, sym_96((&(((*(sym_2583)))._f583)), sym_2586), sym_2587, sym_2588, sym_2589);
        if (((sym_2658)._f567 > 0)) {
            sym_335(sym_2584, sym_2658);
        }
        sym_341(sym_2584);
        return sym_49();
    }
    if ((sym_2591 == (sym_2590)._f892)) {
        uint32_t sym_2659 = sym_95((&(((*(sym_2583)))._f583)), sym_2586);
        if ((sym_2659 != sym_85())) {
            S_23 sym_2660 = sym_575(sym_2583, sym_2584, sym_2585, sym_2659, sym_2587, sym_2588, sym_2589);
            sym_336(sym_2584, sym_2660, true);
        } else {
            sym_336(sym_2584, sym_49(), false);
        }
        return sym_49();
    }
    if ((sym_2591 == (sym_2590)._f899)) {
        uint32_t sym_2661 = sym_571(sym_2583, sym_2586);
        sym_574(sym_2583, sym_2661, sym_2589);
        S_23 sym_2662 = sym_570((&(((*(sym_2583)))._f587)), sym_2585, sym_2661);
        uint32_t sym_2663 = sym_95((&(((*(sym_2583)))._f583)), sym_2586);
        uint32_t sym_2664 = sym_96((&(((*(sym_2583)))._f583)), sym_2586);
        S_430 sym_2665 = sym_217(sym_2583, sym_2663);
        S_430 sym_2666 = sym_217(sym_2583, sym_2664);
        S_568 sym_2667 = sym_56(sym_2585);
        sym_57((&(sym_2667)), sym_50(((uint8_t*)"((")));
        sym_57((&(sym_2667)), sym_2662);
        sym_57((&(sym_2667)), sym_50(((uint8_t*)"){")));
        uint64_t sym_2668 = 0;
        while (((sym_2668 < (sym_2665)._f567) && (sym_2668 < (sym_2666)._f567))) {
            uint32_t sym_2669 = (*(((sym_2665)._f566 + sym_2668)));
            uint32_t sym_2670 = (*(((sym_2666)._f566 + sym_2668)));
            uint32_t sym_2671 = sym_95((&(((*(sym_2583)))._f583)), sym_2669);
            S_23 sym_2672 = sym_575(sym_2583, sym_2584, sym_2585, sym_2670, sym_2587, sym_2588, sym_2589);
            sym_57((&(sym_2667)), sym_50(((uint8_t*)"._f")));
            sym_60((&(sym_2667)), sym_2671);
            sym_57((&(sym_2667)), sym_50(((uint8_t*)" = ")));
            sym_57((&(sym_2667)), sym_2672);
            if (((sym_2668 + 1) < (sym_2665)._f567)) {
                sym_57((&(sym_2667)), sym_50(((uint8_t*)", ")));
            }
            sym_2668 = (sym_2668 + 1);
        }
        sym_57((&(sym_2667)), sym_50(((uint8_t*)"})")));
        return sym_58((&(sym_2667)));
    }
    if ((sym_2591 == (sym_2590)._f902)) {
        uint32_t sym_2673 = sym_95((&(((*(sym_2583)))._f583)), sym_2586);
        uint32_t sym_2674 = sym_96((&(((*(sym_2583)))._f583)), sym_2586);
        if (((sym_2673 == sym_85()) || (sym_2674 == sym_85()))) {
            return sym_49();
        }
        S_23 sym_2675 = sym_575(sym_2583, sym_2584, sym_2585, sym_2673, sym_2587, sym_2588, sym_2589);
        uint32_t sym_2676 = sym_95((&(((*(sym_2583)))._f583)), sym_2674);
        uint32_t sym_2677 = sym_571(sym_2583, sym_2673);
        bool sym_2678 = (sym_260((&(((*(sym_2583)))._f587)), sym_2677) == (sym_234())._f566);
        S_568 sym_2679 = sym_56(sym_2585);
        sym_57((&(sym_2679)), sym_50(((uint8_t*)"(")));
        sym_57((&(sym_2679)), sym_2675);
        if (sym_2678) {
            sym_57((&(sym_2679)), sym_50(((uint8_t*)")->_f")));
        } else {
            sym_57((&(sym_2679)), sym_50(((uint8_t*)")._f")));
        }
        sym_60((&(sym_2679)), sym_2676);
        return sym_58((&(sym_2679)));
    }
    return sym_49();
}

void sym_334(S_58* sym_2682, S_23 sym_2683, S_23 sym_2684, S_23 sym_2685, bool sym_2686) {
    sym_329(sym_2682);
    sym_322((&(((*(sym_2682)))._f625)), sym_2684);
    sym_323((&(((*(sym_2682)))._f625)), ((uint8_t)32));
    sym_322((&(((*(sym_2682)))._f625)), sym_2683);
    if (sym_2686) {
        sym_322((&(((*(sym_2682)))._f625)), sym_50(((uint8_t*)" = ")));
        sym_322((&(((*(sym_2682)))._f625)), sym_2685);
    }
    sym_322((&(((*(sym_2682)))._f625)), sym_50(((uint8_t*)";\n")));
    return;
}

void sym_329(S_58* sym_2687) {
    uint32_t sym_2688 = 0;
    while ((sym_2688 < ((*(sym_2687)))._f629)) {
        sym_322((&(((*(sym_2687)))._f625)), sym_50(((uint8_t*)"    ")));
        sym_2688 = (sym_2688 + 1);
    }
    return;
}

void sym_574(S_57* sym_2689, uint32_t sym_2690, S_45* sym_2691) {
    if ((sym_2690 == sym_236())) {
        return;
    }
    S_112 sym_2692 = sym_234();
    uint8_t sym_2693 = sym_260((&(((*(sym_2689)))._f587)), sym_2690);
    if ((sym_2693 == (sym_2692)._f650)) {
        if ((!(sym_572(sym_2691, sym_2690)))) {
            sym_394(sym_2691, sym_2690);
            S_430 sym_2694 = sym_290((&(((*(sym_2689)))._f587)), sym_2690);
            uint64_t sym_2695 = ((sym_2694)._f567 / 2);
            uint64_t sym_2696 = 0;
            while ((sym_2696 < sym_2695)) {
                uint32_t sym_2697 = (*((((sym_2694)._f566 + (sym_2696 * 2)) + 1)));
                sym_574(sym_2689, sym_2697, sym_2691);
                sym_2696 = (sym_2696 + 1);
            }
        }
    } else {
        if ((sym_2693 == (sym_2692)._f566)) {
            uint32_t sym_2698 = sym_261((&(((*(sym_2689)))._f587)), sym_2690);
            sym_574(sym_2689, sym_2698, sym_2691);
        }
    }
    return;
}

S_23 sym_573(S_351 sym_2699, uint8_t sym_2700) {
    if ((sym_2700 == (sym_2699)._f863)) {
        return sym_50(((uint8_t*)" + "));
    }
    if ((sym_2700 == (sym_2699)._f864)) {
        return sym_50(((uint8_t*)" - "));
    }
    if ((sym_2700 == (sym_2699)._f865)) {
        return sym_50(((uint8_t*)" * "));
    }
    if ((sym_2700 == (sym_2699)._f866)) {
        return sym_50(((uint8_t*)" / "));
    }
    if ((sym_2700 == (sym_2699)._f867)) {
        return sym_50(((uint8_t*)" % "));
    }
    if ((sym_2700 == (sym_2699)._f868)) {
        return sym_50(((uint8_t*)" == "));
    }
    if ((sym_2700 == (sym_2699)._f869)) {
        return sym_50(((uint8_t*)" != "));
    }
    if ((sym_2700 == (sym_2699)._f870)) {
        return sym_50(((uint8_t*)" < "));
    }
    if ((sym_2700 == (sym_2699)._f871)) {
        return sym_50(((uint8_t*)" > "));
    }
    if ((sym_2700 == (sym_2699)._f872)) {
        return sym_50(((uint8_t*)" <= "));
    }
    if ((sym_2700 == (sym_2699)._f873)) {
        return sym_50(((uint8_t*)" >= "));
    }
    if ((sym_2700 == (sym_2699)._f876)) {
        return sym_50(((uint8_t*)" & "));
    }
    if ((sym_2700 == (sym_2699)._f877)) {
        return sym_50(((uint8_t*)" | "));
    }
    if ((sym_2700 == (sym_2699)._f878)) {
        return sym_50(((uint8_t*)" ^ "));
    }
    if ((sym_2700 == (sym_2699)._f879)) {
        return sym_50(((uint8_t*)" << "));
    }
    if ((sym_2700 == (sym_2699)._f880)) {
        return sym_50(((uint8_t*)" >> "));
    }
    if ((sym_2700 == (sym_2699)._f868)) {
        return sym_50(((uint8_t*)" == "));
    }
    if ((sym_2700 == (sym_2699)._f869)) {
        return sym_50(((uint8_t*)" != "));
    }
    if ((sym_2700 == (sym_2699)._f870)) {
        return sym_50(((uint8_t*)" < "));
    }
    if ((sym_2700 == (sym_2699)._f871)) {
        return sym_50(((uint8_t*)" > "));
    }
    if ((sym_2700 == (sym_2699)._f872)) {
        return sym_50(((uint8_t*)" <= "));
    }
    if ((sym_2700 == (sym_2699)._f873)) {
        return sym_50(((uint8_t*)" >= "));
    }
    if ((sym_2700 == (sym_2699)._f876)) {
        return sym_50(((uint8_t*)" & "));
    }
    if ((sym_2700 == (sym_2699)._f877)) {
        return sym_50(((uint8_t*)" | "));
    }
    if ((sym_2700 == (sym_2699)._f878)) {
        return sym_50(((uint8_t*)" ^ "));
    }
    if ((sym_2700 == (sym_2699)._f879)) {
        return sym_50(((uint8_t*)" << "));
    }
    if ((sym_2700 == (sym_2699)._f880)) {
        return sym_50(((uint8_t*)" >> "));
    }
    if ((sym_2700 == (sym_2699)._f874)) {
        return sym_50(((uint8_t*)" && "));
    }
    if ((sym_2700 == (sym_2699)._f875)) {
        return sym_50(((uint8_t*)" || "));
    }
    return sym_50(((uint8_t*)" "));
}

void sym_335(S_58* sym_2701, S_23 sym_2702) {
    if (((sym_2702)._f567 == 0)) {
        return;
    }
    sym_329(sym_2701);
    sym_322((&(((*(sym_2701)))._f625)), sym_2702);
    sym_322((&(((*(sym_2701)))._f625)), sym_50(((uint8_t*)";\n")));
    return;
}

void sym_338(S_58* sym_2703, S_23 sym_2704) {
    sym_329(sym_2703);
    sym_322((&(((*(sym_2703)))._f625)), sym_50(((uint8_t*)"if (")));
    sym_322((&(((*(sym_2703)))._f625)), sym_2704);
    sym_322((&(((*(sym_2703)))._f625)), sym_50(((uint8_t*)") {\n")));
    ((*(sym_2703)))._f629 = (((*(sym_2703)))._f629 + 1);
    return;
}

void sym_339(S_58* sym_2705) {
    if ((((*(sym_2705)))._f629 > 0)) {
        ((*(sym_2705)))._f629 = (((*(sym_2705)))._f629 - 1);
    }
    sym_329(sym_2705);
    sym_322((&(((*(sym_2705)))._f625)), sym_50(((uint8_t*)"} else {\n")));
    ((*(sym_2705)))._f629 = (((*(sym_2705)))._f629 + 1);
    return;
}

void sym_341(S_58* sym_2706) {
    if ((((*(sym_2706)))._f629 > 0)) {
        ((*(sym_2706)))._f629 = (((*(sym_2706)))._f629 - 1);
    }
    sym_329(sym_2706);
    sym_322((&(((*(sym_2706)))._f625)), sym_50(((uint8_t*)"}\n")));
    return;
}

void sym_340(S_58* sym_2707, S_23 sym_2708) {
    sym_329(sym_2707);
    sym_322((&(((*(sym_2707)))._f625)), sym_50(((uint8_t*)"while (")));
    sym_322((&(((*(sym_2707)))._f625)), sym_2708);
    sym_322((&(((*(sym_2707)))._f625)), sym_50(((uint8_t*)") {\n")));
    ((*(sym_2707)))._f629 = (((*(sym_2707)))._f629 + 1);
    return;
}

void sym_336(S_58* sym_2709, S_23 sym_2710, bool sym_2711) {
    sym_329(sym_2709);
    sym_322((&(((*(sym_2709)))._f625)), sym_50(((uint8_t*)"return")));
    if ((sym_2711 && ((sym_2710)._f567 > 0))) {
        sym_323((&(((*(sym_2709)))._f625)), ((uint8_t)32));
        sym_322((&(((*(sym_2709)))._f625)), sym_2710);
    }
    sym_322((&(((*(sym_2709)))._f625)), sym_50(((uint8_t*)";\n")));
    return;
}

void sym_333(S_58* sym_2712) {
    if ((((*(sym_2712)))._f629 > 0)) {
        ((*(sym_2712)))._f629 = (((*(sym_2712)))._f629 - 1);
    }
    sym_322((&(((*(sym_2712)))._f625)), sym_50(((uint8_t*)"}\n\n")));
    return;
}

S_45 sym_596(S_54* sym_2713, S_45* sym_2714) {
    S_45 sym_2715 = sym_390();
    S_42 sym_2716 = sym_348();
    uint32_t sym_2717 = sym_259(sym_2713);
    uint32_t sym_2718 = 0;
    while ((sym_2718 < sym_2717)) {
        sym_352((&(sym_2716)), ((uint8_t)0));
        sym_2718 = (sym_2718 + 1);
    }
    uint64_t sym_2719 = 0;
    while ((sym_2719 < ((*(sym_2714)))._f567)) {
        uint32_t sym_2720 = sym_396(sym_2714, sym_2719);
        sym_595(sym_2713, (&(sym_2715)), (&(sym_2716)), sym_2720);
        sym_2719 = (sym_2719 + 1);
    }
    sym_350((&(sym_2716)));
    return sym_2715;
}

uint32_t sym_259(S_54* sym_2721) {
    return sym_249((&(((*(sym_2721)))._f612)));
}

void sym_595(S_54* sym_2722, S_45* sym_2723, S_42* sym_2724, uint32_t sym_2725) {
    if ((((uint64_t)sym_2725) >= ((*(sym_2724)))._f567)) {
        return;
    }
    uint8_t sym_2726 = sym_354(sym_2724, ((uint64_t)sym_2725));
    if ((sym_2726 != ((uint8_t)0))) {
        return;
    }
    sym_355(sym_2724, ((uint64_t)sym_2725), ((uint8_t)1));
    S_112 sym_2727 = sym_234();
    uint8_t sym_2728 = sym_260(sym_2722, sym_2725);
    if ((sym_2728 == (sym_2727)._f650)) {
        S_430 sym_2729 = sym_290(sym_2722, sym_2725);
        uint64_t sym_2730 = ((sym_2729)._f567 / 2);
        uint64_t sym_2731 = 0;
        while ((sym_2731 < sym_2730)) {
            uint32_t sym_2732 = (*((((sym_2729)._f566 + (sym_2731 * 2)) + 1)));
            uint8_t sym_2733 = sym_260(sym_2722, sym_2732);
            if (((sym_2733 == (sym_2727)._f650) || (sym_2733 == (sym_2727)._f651))) {
                sym_595(sym_2722, sym_2723, sym_2724, sym_2732);
            }
            sym_2731 = (sym_2731 + 1);
        }
    } else {
        if ((sym_2728 == (sym_2727)._f651)) {
            uint32_t sym_2734 = sym_283(sym_2722, sym_2725);
            uint8_t sym_2735 = sym_260(sym_2722, sym_2734);
            if (((sym_2735 == (sym_2727)._f650) || (sym_2735 == (sym_2727)._f651))) {
                sym_595(sym_2722, sym_2723, sym_2724, sym_2734);
            }
        }
    }
    sym_394(sym_2723, sym_2725);
    sym_355(sym_2724, ((uint64_t)sym_2725), ((uint8_t)2));
    return;
}

void sym_350(S_42* sym_2736) {
    sym_369(sym_2736);
    return;
}

void sym_369(S_42* sym_2737) {
    free(((uint8_t*)((*(sym_2737)))._f566));
    ((*(sym_2737)))._f566 = ((uint8_t*)0);
    ((*(sym_2737)))._f567 = 0;
    ((*(sym_2737)))._f554 = 0;
    return;
}

void sym_343(S_58* sym_2738, uint32_t sym_2739, S_23 sym_2740, S_23 sym_2741) {
    uint64_t sym_2742 = 0;
    while ((sym_2742 < (((*(sym_2738)))._f626)._f567)) {
        if ((sym_396((&(((*(sym_2738)))._f626)), sym_2742) == sym_2739)) {
            return;
        }
        sym_2742 = (sym_2742 + 1);
    }
    sym_394((&(((*(sym_2738)))._f626)), sym_2739);
    sym_342(sym_2738, sym_2740);
    sym_322((&(((*(sym_2738)))._f623)), sym_50(((uint8_t*)"struct ")));
    sym_322((&(((*(sym_2738)))._f623)), sym_2740);
    sym_322((&(((*(sym_2738)))._f623)), sym_50(((uint8_t*)" {\n")));
    sym_322((&(((*(sym_2738)))._f623)), sym_2741);
    sym_322((&(((*(sym_2738)))._f623)), sym_50(((uint8_t*)"};\n\n")));
    return;
}

void sym_342(S_58* sym_2743, S_23 sym_2744) {
    sym_322((&(((*(sym_2743)))._f622)), sym_50(((uint8_t*)"typedef struct ")));
    sym_322((&(((*(sym_2743)))._f622)), sym_2744);
    sym_323((&(((*(sym_2743)))._f622)), ((uint8_t)32));
    sym_322((&(((*(sym_2743)))._f622)), sym_2744);
    sym_322((&(((*(sym_2743)))._f622)), sym_50(((uint8_t*)";\n")));
    return;
}

S_23 sym_337(S_58* sym_2745, S_23 sym_2746) {
    S_42 sym_2747 = sym_348();
    sym_322((&(sym_2747)), sym_2746);
    sym_322((&(sym_2747)), sym_50(((uint8_t*)"// Struct Declarations\n")));
    sym_322((&(sym_2747)), sym_325((&(((*(sym_2745)))._f622))));
    sym_322((&(sym_2747)), sym_50(((uint8_t*)"\n// Struct Definitions\n")));
    sym_322((&(sym_2747)), sym_325((&(((*(sym_2745)))._f623))));
    sym_322((&(sym_2747)), sym_50(((uint8_t*)"\n// Forward Declarations\n")));
    sym_322((&(sym_2747)), sym_325((&(((*(sym_2745)))._f624))));
    sym_322((&(sym_2747)), sym_50(((uint8_t*)"\n// Implementations\n")));
    sym_322((&(sym_2747)), sym_325((&(((*(sym_2745)))._f625))));
    if (((*(sym_2745)))._f628) {
        sym_322((&(sym_2747)), sym_50(((uint8_t*)"int main(void) {\n    sym_")));
        sym_324((&(sym_2747)), ((*(sym_2745)))._f627);
        sym_322((&(sym_2747)), sym_50(((uint8_t*)"();\n    return 0;\n}\n")));
    }
    sym_323((&(sym_2747)), ((uint8_t)0));
    return sym_325((&(sym_2747)));
}

S_23 sym_325(S_42* sym_2748) {
    return ((S_23){._f566 = ((*(sym_2748)))._f566, ._f567 = ((*(sym_2748)))._f567});
}

void sym_324(S_42* sym_2749, uint32_t sym_2750) {
    if ((sym_2750 == 0)) {
        sym_352(sym_2749, ((uint8_t)48));
        return;
    }
    uint32_t sym_2751 = 1;
    uint32_t sym_2752 = sym_2750;
    while ((sym_2752 >= 10)) {
        sym_2751 = (sym_2751 * 10);
        sym_2752 = (sym_2752 / 10);
    }
    uint32_t sym_2753 = sym_2750;
    while ((sym_2751 > 0)) {
        uint8_t sym_2754 = ((uint8_t)(sym_2753 / sym_2751));
        sym_352(sym_2749, (((uint8_t)48) + sym_2754));
        sym_2753 = (sym_2753 - (((uint32_t)sym_2754) * sym_2751));
        sym_2751 = (sym_2751 / 10);
    }
    return;
}

bool sym_83(uint8_t* sym_2755, S_23 sym_2756) {
    uint8_t* sym_2757 = file_open(sym_2755, ((uint8_t*)"wb"));
    if ((sym_2757 == ((uint8_t*)0))) {
        return false;
    }
    uint64_t sym_2758 = file_write(sym_2757, (sym_2756)._f566, 1, (sym_2756)._f567);
    file_close(sym_2757);
    return (sym_2758 == (sym_2756)._f567);
}

int main(void) {
    sym_6();
    return 0;
}
 