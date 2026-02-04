//! Module: builtin
//!
//! Contains 159 transpiled functions:
//! - f_min_by_impl:8766168783804090136:./src/builtin.c
//! - type_error:1545696330699923143:./src/builtin.c
//! - f_sqrt:4410889880684941690:./src/builtin.c
//! - f_fmod:9152118536536145527:./src/builtin.c
//! - f_get_prog_origin:15295976106568957150:./src/builtin.c
//! - binop_lesseq:15100826914301278088:./src/builtin.c
//! - f_halt_error:5084479183958820906:./src/builtin.c
//! - f_get_jq_origin:825656245463824909:./src/builtin.c
//! - f_nextafter:18068440426015583896:./src/builtin.c
//! - f_fdim:7544605882751460665:./src/builtin.c
//! - gen_builtin_list:11008291039522284700:./src/builtin.c
//! - f_j0:4875529634569671248:./src/builtin.c
//! - f_nearbyint:11035198959138066518:./src/builtin.c
//! - f_utf8bytelength:4037683450117977895:./src/builtin.c
//! - f_fma:17778999399180013883:./src/builtin.c
//! - f_keys_unsorted:10334473657648569844:./src/builtin.c
//! - f_sort:8728498694946718595:./src/builtin.c
//! - binop_greatereq:4592631139589798378:./src/builtin.c
//! - ret_error2:14393307849896372160:./src/builtin.c
//! - f_max_by_impl:10720737648057076161:./src/builtin.c
//! - f_have_decnum:6061897299321200814:./src/builtin.c
//! - f_atan:3212188079769475775:./src/builtin.c
//! - tm2jv:10763538098429592265:./src/builtin.c
//! - f_env:18194521517419054495:./src/builtin.c
//! - escape_string:6949101433822029845:./src/builtin.c
//! - f_fmin:14144145414615266437:./src/builtin.c
//! - f_acos:2578369155283336750:./src/builtin.c
//! - f_infinite:5302041726166403405:./src/builtin.c
//! - f_pow:14614586456475415149:./src/builtin.c
//! - binop_notequal:6447805051014175874:./src/builtin.c
//! - f_nan:9141447258128709237:./src/builtin.c
//! - __jq_significand:6880562240638705015:./src/builtin.c
//! - f_string_trim:6616807825132237779:./src/builtin.c
//! - f_cos:3529408317474096965:./src/builtin.c
//! - f_remainder:17188389484702650331:./src/builtin.c
//! - f_notequal:14695990949669246964:./src/builtin.c
//! - f_y0:5971198217564959078:./src/builtin.c
//! - type_error2:11127081542255890484:./src/builtin.c
//! - ret_error:7289099005972057367:./src/builtin.c
//! - f_match:15005301602270608770:./src/builtin.c
//! - f_tostring:7825694201860072108:./src/builtin.c
//! - f_max:1489408677019344698:./src/builtin.c
//! - f_plus:9412348551371531165:./src/builtin.c
//! - builtins_bind:2807742937553337692:./src/builtin.c
//! - f_delpaths:6582245522006822206:./src/builtin.c
//! - minmax_by:16275525043853891448:./src/builtin.c
//! - f_modf:2726053406219318067:./src/builtin.c
//! - f_fmax:498825844322524126:./src/builtin.c
//! - f_greater:5916992070533041534:./src/builtin.c
//! - f_frexp:1206981792469123349:./src/builtin.c
//! - f_halt:10036121401672174478:./src/builtin.c
//! - f_mktime:12849959255982696110:./src/builtin.c
//! - f_scalb:9924480237756356465:./src/builtin.c
//! - my_mktime:5535894628297861373:./src/builtin.c
//! - f_y1:11599594088723353909:./src/builtin.c
//! - f_isnormal:3356901286147685878:./src/builtin.c
//! - f_multiply:6099981490885371312:./src/builtin.c
//! - f_jn:15965798135947284766:./src/builtin.c
//! - f_atan2:16650124450020857627:./src/builtin.c
//! - f_string_indexes:11969060271236981979:./src/builtin.c
//! - f_trunc:1548595960936558437:./src/builtin.c
//! - f_asinh:1515872503191391927:./src/builtin.c
//! - f_greatereq:10261915084503232406:./src/builtin.c
//! - binop_greater:11266875320094174208:./src/builtin.c
//! - f_json_parse:1097809511634651530:./src/builtin.c
//! - f_get_search_list:8293098009128063862:./src/builtin.c
//! - binop_plus:6008934415828237994:./src/builtin.c
//! - f_type:13909205933518105874:./src/builtin.c
//! - f_current_filename:1981637727679252501:./src/builtin.c
//! - f_fabs:17609145309434748704:./src/builtin.c
//! - f_exp10:8064278896630551597:./src/builtin.c
//! - f_string_rtrim:2555011487471295444:./src/builtin.c
//! - f_tanh:17113597112385258681:./src/builtin.c
//! - f_acosh:6851547750627553898:./src/builtin.c
//! - f_logb:9233833843916387070:./src/builtin.c
//! - f_contains:17055964957982577672:./src/builtin.c
//! - f_modulemeta:10662026737477008563:./src/builtin.c
//! - f_drem:8178950485508568369:./src/builtin.c
//! - f_keys:18219451660179733309:./src/builtin.c
//! - f_input:4880840103251136690:./src/builtin.c
//! - f_yn:8709484303238691304:./src/builtin.c
//! - f_equal:14417984421264330263:./src/builtin.c
//! - f_isinfinite:840800251128108633:./src/builtin.c
//! - f_negate:1205918829884187446:./src/builtin.c
//! - f_tan:13839396204485494902:./src/builtin.c
//! - f_exp:12919555618747136942:./src/builtin.c
//! - f_log10:17127887329213273624:./src/builtin.c
//! - f_minus:16612776882576161693:./src/builtin.c
//! - f_sinh:2299451994753815875:./src/builtin.c
//! - f_lesseq:13177842036430395590:./src/builtin.c
//! - f_sort_by_impl:9018484491961547412:./src/builtin.c
//! - f_gmtime:5043702891083959533:./src/builtin.c
//! - f_j1:9330841675033436984:./src/builtin.c
//! - f_has:14631822450577061667:./src/builtin.c
//! - f_divide:16795399416822020735:./src/builtin.c
//! - f_strftime:1990117810479121898:./src/builtin.c
//! - binop_mod:1153368054341086795:./src/builtin.c
//! - f_getpath:8247338710997153676:./src/builtin.c
//! - f_tgamma:1506801877624250245:./src/builtin.c
//! - f_strptime:13517879006751595099:./src/builtin.c
//! - f_significand:10923409861356429764:./src/builtin.c
//! - f_cbrt:15673619283097700866:./src/builtin.c
//! - f_isnan:1195330915193707632:./src/builtin.c
//! - f_nexttoward:2498238336356996045:./src/builtin.c
//! - f_ldexp:8018963428610132225:./src/builtin.c
//! - f_length:1942312530824406228:./src/builtin.c
//! - f_hypot:13341059103976983234:./src/builtin.c
//! - f_rint:14287202101746569239:./src/builtin.c
//! - bind_bytecoded_builtins:16428135527618916709:./src/builtin.c
//! - f_atanh:3633388727829880683:./src/builtin.c
//! - f_string_implode:5363830993849100805:./src/builtin.c
//! - f_floor:7311308032749631858:./src/builtin.c
//! - f_dump:15710677104011096399:./src/builtin.c
//! - f_min:10865877687566922679:./src/builtin.c
//! - f_exp2:18127241044791737814:./src/builtin.c
//! - f_cosh:6987892680971487453:./src/builtin.c
//! - f_format:16057494096127281969:./src/builtin.c
//! - f_mod:10098875485270085924:./src/builtin.c
//! - f_log1p:6862003472287093208:./src/builtin.c
//! - f_asin:1175201248337231844:./src/builtin.c
//! - binop_less:15674423134712629328:./src/builtin.c
//! - set_tm_yday:5413331944941743299:./src/builtin.c
//! - set_tm_wday:9050273483569096741:./src/builtin.c
//! - f_debug:8000381746088605266:./src/builtin.c
//! - f_erf:8439294318749196055:./src/builtin.c
//! - f_ceil:9852466063697202125:./src/builtin.c
//! - f_error:11855585450036744095:./src/builtin.c
//! - f_gamma:13026085375308435690:./src/builtin.c
//! - f_lgamma_r:15116237145195212653:./src/builtin.c
//! - f_stderr:17240111983795952998:./src/builtin.c
//! - f_less:7893365630656790649:./src/builtin.c
//! - binop_divide:2650628427002585332:./src/builtin.c
//! - f_localtime:1305550169983196351:./src/builtin.c
//! - f_now:13801689251032971042:./src/builtin.c
//! - binop_equal:4415354057817293775:./src/builtin.c
//! - f_strflocaltime:6519095094343429599:./src/builtin.c
//! - binop_minus:2708106700348330474:./src/builtin.c
//! - f_copysign:17081262452966914851:./src/builtin.c
//! - jv2tm:12555262797125427344:./src/builtin.c
//! - f_string_split:5422250538248208758:./src/builtin.c
//! - f_string_ltrim:15352832974260617065:./src/builtin.c
//! - f_startswith:9795427360762556599:./src/builtin.c
//! - order_cmp:5481290025931680481:./src/builtin.c
//! - f_scalbln:16179442862723930341:./src/builtin.c
//! - f_erfc:6309906728259442166:./src/builtin.c
//! - binop_multiply:1476492886223898803:./src/builtin.c
//! - f_group_by_impl:17037255368836125304:./src/builtin.c
//! - f_setpath:15756628105770875548:./src/builtin.c
//! - f_expm1:2421053383388711227:./src/builtin.c
//! - f_lgamma:16696872795656342179:./src/builtin.c
//! - f_current_line:14186940603620411187:./src/builtin.c
//! - f_tonumber:8624085869459664897:./src/builtin.c
//! - f_log:17573349348990829073:./src/builtin.c
//! - f_endswith:6964994331449190694:./src/builtin.c
//! - f_log2:6202198152771359779:./src/builtin.c
//! - f_sin:5602159843502873341:./src/builtin.c
//! - f_round:9761739005879123076:./src/builtin.c
//! - string_trim:3583471767544888104:./src/builtin.c
//! - f_string_explode:15454606891065300608:./src/builtin.c

use crate::jv_aux::{jv_cmp, jv_has, jv_setpath, jv_sort, jv_keys_unsorted, jv_group, jv_get, jv_set, jv_getpath, jv_keys, jv_delpaths};
use crate::execute::{jq_get_prog_origin, jq_halt, jq_get_jq_origin, jq_get_debug_cb, _jq_path_append, jq_get_lib_dirs, jq_get_input_cb};
use std::rc::Rc;
use std::cell::RefCell;
use crate::jv::{Jv, JvKind, jv_array_append, jv_string_split, jv_string_explode};
use std::time::{SystemTime, UNIX_EPOCH};
use std::cmp::Ordering;
use std::collections::HashMap;
use std::env;
use std::f64::consts::PI;
// Note: strptime-like functionality is provided by strptime_simple locally
use crate::inject_errors::fwrite;
use crate::types::*;
// Import types that are re-exported privately from other modules directly
use crate::types::{JqState, Locfile};
// Note: type_error, binop_notequal, binop_plus, order_cmp, CmpOp are defined in this file

// ============================================================================
// Embedded builtin.jq content - matches C's jq_builtins[] in builtin.c
// This is parsed by jq_parse_library to provide jq-defined builtin functions
// ============================================================================
pub const JQ_BUILTINS: &str = r#"def halt_error: halt_error(5);
def error(msg): msg|error;
def map(f): [.[] | f];
def select(f): if f then . else empty end;
def sort_by(f): _sort_by_impl(map([f]));
def group_by(f): _group_by_impl(map([f]));
def unique: group_by(.) | map(.[0]);
def unique_by(f): group_by(f) | map(.[0]);
def max_by(f): _max_by_impl(map([f]));
def min_by(f): _min_by_impl(map([f]));
def add: reduce .[] as $x (null; . + $x);
def del(f): delpaths([path(f)]);
def abs: if . < 0 then - . else . end;
def _assign(paths; $value): reduce path(paths) as $p (.; setpath($p; $value));
def _modify(paths; update):
    reduce path(paths) as $p ([., []];
        . as $dot
      | null
      | label $out
      | ($dot[0] | getpath($p)) as $v
      | (
          (   $$$$v
            | update
            | (., break $out) as $v
            | $$$$dot
            | setpath([0] + $p; $v)
          ),
          (
              $$$$dot
            | setpath([1, (.[1] | length)]; $p)
          )
        )
    ) | . as $dot | $dot[0] | delpaths($dot[1]);
def map_values(f): .[] |= f;
def recurse(f): def r: ., (f | r); r;
def recurse(f; cond): def r: ., (f | select(cond) | r); r;
def recurse: recurse(.[]?);
def to_entries: [keys_unsorted[] as $k | {key: $k, value: .[$k]}];
def from_entries: map({(.key // .Key // .name // .Name): (if has("value") then .value else .Value end)}) | add | .//={};
def with_entries(f): to_entries | map(f) | from_entries;
def reverse: [.[length - 1 - range(0;length)]];
def indices($i): if type == "array" and ($i|type) == "array" then .[$i]
  elif type == "array" then .[[$i]]
  elif type == "string" and ($i|type) == "string" then _strindices($i)
  else .[$i] end;
def index($i):   indices($i) | .[0];
def rindex($i):  indices($i) | .[-1:][0];
def paths: path(recurse)|select(length > 0);
def paths(node_filter): path(recurse|select(node_filter))|select(length > 0);
def isfinite: type == "number" and (isinfinite | not);
def arrays: select(type == "array");
def objects: select(type == "object");
def iterables: select(type|. == "array" or . == "object");
def booleans: select(type == "boolean");
def numbers: select(type == "number");
def normals: select(isnormal);
def finites: select(isfinite);
def strings: select(type == "string");
def nulls: select(. == null);
def values: select(. != null);
def scalars: select(type|. != "array" and . != "object");
def join($x): reduce .[] as $i (null;
            (if .==null then "" else .+$x end) +
            ($i | if type=="boolean" or type=="number" then tostring else .//"" end)
        ) // "";
def _flatten($x): reduce .[] as $i ([]; if $i | type == "array" and $x != 0 then . + ($i | _flatten($x-1)) else . + [$i] end);
def flatten($x): if $x < 0 then error("flatten depth must not be negative") else _flatten($x) end;
def flatten: _flatten(-1);
def range($x): range(0;$x);
def fromdateiso8601: strptime("%Y-%m-%dT%H:%M:%SZ")|mktime;
def todateiso8601: strftime("%Y-%m-%dT%H:%M:%SZ");
def fromdate: fromdateiso8601;
def todate: todateiso8601;
def ltrimstr($left): if startswith($left) then .[$left | length:] end;
def rtrimstr($right): if endswith($right) then .[:$right | -length] end;
def match(re; mode): _match_impl(re; mode; false)|.[];
def match($val): ($val|type) as $vt | if $vt == "string" then match($val; null)
   elif $vt == "array" and ($val | length) > 1 then match($val[0]; $val[1])
   elif $vt == "array" and ($val | length) > 0 then match($val[0]; null)
   else error( $vt + " not a string or array") end;
def test(re; mode): _match_impl(re; mode; true);
def test($val): ($val|type) as $vt | if $vt == "string" then test($val; null)
   elif $vt == "array" and ($val | length) > 1 then test($val[0]; $val[1])
   elif $vt == "array" and ($val | length) > 0 then test($val[0]; null)
   else error( $vt + " not a string or array") end;
def capture(re; mods): match(re; mods) | reduce ( .captures | .[] | select(.name != null) | { (.name) : .string } ) as $pair ({}; . + $pair);
def capture($val): ($val|type) as $vt | if $vt == "string" then capture($val; null)
   elif $vt == "array" and ($val | length) > 1 then capture($val[0]; $val[1])
   elif $vt == "array" and ($val | length) > 0 then capture($val[0]; null)
   else error( $vt + " not a string or array") end;
def scan($re; $flags):
  match($re; "g" + $flags)
    | if (.captures|length > 0)
      then [ .captures | .[] | .string ]
      else .string
      end;
def scan($re): scan($re; null);
def _nwise($n):
  def n: if length <= $n then . else .[0:$n] , (.[$n:] | n) end;
  n;
def _nwise(a; $n): a | _nwise($n);
def splits($re; flags): . as $s
  | [ match($re; "g" + flags) | (.offset, .offset + .length) ]
  | [0] + . +[$s|length]
  | _nwise(2)
  | $s[.[0]:.[1] ] ;
def splits($re): splits($re; null);
def split($re; flags): [ splits($re; flags) ];
def sub($re; s; $flags):
   . as $in
   | (reduce match($re; $flags) as $edit
        ({result: [], previous: 0};
            $in[ .previous: ($edit | .offset) ] as $gap
            | [reduce ( $edit | .captures | .[] | select(.name != null) | { (.name) : .string } ) as $pair
                 ({}; . + $pair) | s ] as $inserts
            | reduce range(0; $inserts|length) as $ix (.; .result[$ix] += $gap + $inserts[$ix])
            | .previous = ($edit | .offset + .length ) )
          | .result[] + $in[.previous:] )
      // $in;
def sub($re; s): sub($re; s; "");
def gsub($re; s; flags): sub($re; s; flags + "g");
def gsub($re; s): sub($re; s; "g");
def while(cond; update):
     def _while:
         if cond then ., (update | _while) else empty end;
     _while;
def until(cond; next):
     def _until:
         if cond then . else (next|_until) end;
     _until;
def limit($n; exp):
    if $n > 0 then label $out | foreach exp as $item ($n; .-1; $item, if . <= 0 then break $out else empty end)
    elif $n == 0 then empty
    else exp end;
def range($init; $upto; $by):
    if $by > 0 then $init|while(. < $upto; . + $by)
  elif $by < 0 then $init|while(. > $upto; . + $by)
  else empty end;
def first(g): label $out | g | ., break $out;
def isempty(g): first((g|false), true);
def all(generator; condition): isempty(generator|condition and empty);
def any(generator; condition): isempty(generator|condition or empty)|not;
def all(condition): all(.[]; condition);
def any(condition): any(.[]; condition);
def all: all(.[]; .);
def any: any(.[]; .);
def last(g): reduce g as $item (null; $item);
def nth($n; g):
  if $n < 0 then error("nth doesn't support negative indices")
  else label $out | foreach g as $item ($n + 1; . - 1; if . <= 0 then $item, break $out else empty end) end;
def first: .[0];
def last: .[-1];
def nth($n): .[$n];
def combinations:
    if length == 0 then [] else
        .[0][] as $x
          | (.[1:] | combinations) as $y
          | [$x] + $y
    end;
def combinations(n):
    . as $dot
      | [range(n) | $dot]
      | combinations;
def transpose: [range(0; map(length)|max // 0) as $i | [.[][$i]]];
def in(xs): . as $x | xs | has($x);
def inside(xs): . as $x | xs | contains($x);
def repeat(exp):
     def _repeat:
         exp, _repeat;
     _repeat;
def inputs: try repeat(input) catch if .=="break" then empty else error end;
def ascii_downcase:
  explode | map( if 65 <= . and . <= 90 then . + 32  else . end) | implode;
def ascii_upcase:
  explode | map( if 97 <= . and . <= 122 then . - 32  else . end) | implode;
def truncate_stream(stream):
  . as $n | null | stream | . as $input | if (.[0]|length) > $n then setpath([0];$input[0][$n:]) else empty end;
def fromstream(i): {x: null, e: false} as $init |
  foreach i as $i ($init
  ; if .e then $init else . end
  | if $i|length == 2
    then setpath(["e"]; $i[0]|length==0) | setpath(["x"]+$i[0]; $i[1])
    else setpath(["e"]; $i[0]|length==1) end
  ; if .e then .x else empty end);
def tostream:
  path(def r: (.[]?|r), .; r) as $p |
  getpath($p) |
  reduce path(.[]?) as $q ([$p, .]; [$p+$q]);
def bsearch($target):
  if length == 0 then -1
  elif length == 1 then
     if $target == .[0] then 0 elif $target < .[0] then -1 else -2 end
  else . as $in
    | [0, length-1, null]
    | until( .[0] > .[1] ;
             if .[2] != null then (.[1] = -1)
             else
               ( ( (.[1] + .[0]) / 2 ) | floor ) as $mid
               | $in[$mid] as $monkey
               | if $monkey == $target  then (.[2] = $mid)
                 elif .[0] == .[1]     then (.[1] = -1)
                 elif $monkey < $target then (.[0] = ($mid + 1))
                 else (.[1] = ($mid - 1))
                 end
             end )
    | if .[2] == null then
         if $in[ .[0] ] < $target then (-2 -.[0])
         else (-1 -.[0])
         end
      else .[2]
      end
  end;
def walk(f):
  def w:
    if type == "object"
    then map_values(w)
    elif type == "array" then map(w)
    else .
    end
    | f;
  w;
def pick(pathexps):
  . as $in
  | reduce path(pathexps) as $a (null;
      setpath($a; $in|getpath($a)) );
def debug(msgs): (msgs | debug | empty), .;
def INDEX(stream; idx_expr):
  reduce stream as $row ({}; .[$row|idx_expr|tostring] = $row);
def INDEX(idx_expr): INDEX(.[]; idx_expr);
def JOIN($idx; idx_expr):
  [.[] | [., $idx[idx_expr]]];
def JOIN($idx; stream; idx_expr):
  stream | [., $idx[idx_expr]];
def JOIN($idx; stream; idx_expr; join_expr):
  stream | [., $idx[idx_expr]] | join_expr;
def IN(s): any(s == .; .);
def IN(src; s): any(src == s; .);
"#;

// ============================================================================
// Pure Rust implementations of libm functions
// ============================================================================

/// ldexp: x * 2^exp
fn ldexp_impl(x: f64, exp: i32) -> f64 {
    x * 2.0f64.powi(exp)
}

/// frexp: decompose x into mantissa and exponent (x = mantissa * 2^exp)
fn frexp_impl(x: f64) -> (f64, i32) {
    if x == 0.0 || x.is_nan() || x.is_infinite() {
        return (x, 0);
    }
    let bits = x.to_bits();
    let sign = bits & 0x8000_0000_0000_0000;
    let exp = ((bits >> 52) & 0x7FF) as i32;
    let mantissa_bits = bits & 0x000F_FFFF_FFFF_FFFF;

    if exp == 0 {
        // Denormalized number - normalize it first
        let normalized = x * 2.0f64.powi(52);
        let (m, e) = frexp_impl(normalized);
        return (m, e - 52);
    }

    // mantissa in [0.5, 1.0)
    let new_bits = sign | 0x3FE0_0000_0000_0000 | mantissa_bits;
    (f64::from_bits(new_bits), exp - 1022)
}

/// lgamma: log of the absolute value of the gamma function
fn lgamma_impl(x: f64) -> f64 {
    // Lanczos approximation
    if x <= 0.0 && x.floor() == x {
        return f64::INFINITY;
    }
    let xx = if x < 0.5 { 1.0 - x } else { x };
    const G: f64 = 7.0;
    const C: [f64; 9] = [
        0.99999999999980993,
        676.5203681218851,
        -1259.1392167224028,
        771.32342877765313,
        -176.61502916214059,
        12.507343278686905,
        -0.13857109526572012,
        9.9843695780195716e-6,
        1.5056327351493116e-7,
    ];
    let mut sum = C[0];
    for i in 1..9 {
        sum += C[i] / (xx + (i as f64) - 1.0);
    }
    let t = xx + G - 0.5;
    let result = 0.5 * (2.0 * std::f64::consts::PI).ln() + (xx - 0.5) * t.ln() - t + sum.ln();
    if x < 0.5 {
        (std::f64::consts::PI / ((std::f64::consts::PI * x).sin())).ln() - result
    } else {
        result
    }
}

/// tgamma: the gamma function
fn tgamma_impl(x: f64) -> f64 {
    if x <= 0.0 && x.floor() == x {
        return f64::NAN;
    }
    if x == 0.0 {
        return if x.is_sign_positive() { f64::INFINITY } else { f64::NEG_INFINITY };
    }
    lgamma_impl(x).exp() * if x > 0.0 || (x.floor() as i64) % 2 == 0 { 1.0 } else { -1.0 }
}

/// erfc: complementary error function
fn erfc_impl(x: f64) -> f64 {
    // Approximation using Horner's method
    let t = 1.0 / (1.0 + 0.5 * x.abs());
    let tau = t * (-x * x - 1.26551223 +
        t * (1.00002368 +
            t * (0.37409196 +
                t * (0.09678418 +
                    t * (-0.18628806 +
                        t * (0.27886807 +
                            t * (-1.13520398 +
                                t * (1.48851587 +
                                    t * (-0.82215223 +
                                        t * 0.17087277))))))))).exp();
    if x >= 0.0 { tau } else { 2.0 - tau }
}

/// scalbln: x * 2^n (long exponent version)
fn scalbln_impl(x: f64, n: i64) -> f64 {
    // Clamp to i32 range for powi
    let n = n.clamp(i32::MIN as i64, i32::MAX as i64) as i32;
    x * 2.0f64.powi(n)
}

/// jn: Bessel function of the first kind of integer order n
fn jn_impl(n: i32, x: f64) -> f64 {
    if n == 0 {
        return bessel_j0(x);
    }
    if n == 1 {
        return bessel_j1(x);
    }
    if x == 0.0 {
        return 0.0;
    }
    let n_abs = n.abs();
    // Miller's algorithm for recurrence
    let mut bjm: f64;
    let mut bj: f64;
    let mut bjp: f64;
    if x.abs() > n_abs as f64 {
        // Forward recurrence
        bjm = bessel_j0(x);
        bj = bessel_j1(x);
        for j in 1..n_abs {
            bjp = (2 * j) as f64 / x * bj - bjm;
            bjm = bj;
            bj = bjp;
        }
        if n < 0 && n % 2 != 0 {
            return -bj;
        }
        return bj;
    }
    // Backward recurrence
    let m = 2 * ((n_abs + (40.0f64.sqrt() * (n_abs as f64).sqrt()) as i32) / 2);
    let mut tox = 2.0 / x;
    let mut bjp = 0.0;
    bj = 1.0;
    let mut sum = 0.0;
    let mut result = 0.0;
    for j in (1..=m).rev() {
        bjm = j as f64 * tox * bj - bjp;
        bjp = bj;
        bj = bjm;
        if bj.abs() > 1e10 {
            bj *= 1e-10;
            bjp *= 1e-10;
            sum *= 1e-10;
            result *= 1e-10;
        }
        if j % 2 != 0 {
            sum += bj;
        }
        if j == n_abs {
            result = bjp;
        }
    }
    sum = 2.0 * sum - bj;
    result /= sum;
    if n < 0 && n % 2 != 0 {
        -result
    } else {
        result
    }
}

/// yn: Bessel function of the second kind of integer order n
fn yn_impl(n: i32, x: f64) -> f64 {
    if x <= 0.0 {
        return f64::NAN;
    }
    if n == 0 {
        return bessel_y0(x);
    }
    if n == 1 {
        return bessel_y1(x);
    }
    let n_abs = n.abs();
    let mut bym = bessel_y0(x);
    let mut by = bessel_y1(x);
    let tox = 2.0 / x;
    for j in 1..n_abs {
        let byp = j as f64 * tox * by - bym;
        bym = by;
        by = byp;
    }
    if n < 0 && n % 2 != 0 {
        -by
    } else {
        by
    }
}

// ============================================================================
// End of libm replacements
// ============================================================================

/// Helper function to free a jv value (no-op in Rust due to RAII)
fn jv_free(_v: Jv) {}
/// Helper function to copy a jv value
fn jv_copy(v: &Jv) -> Jv {
    v.clone()
}
/// Get the kind of a jv value
pub fn jv_get_kind(v: &Jv) -> JvKind {
    match v.kind_flags & 0x0F {
        0 => JvKind::Invalid,
        1 => JvKind::Null,
        2 => JvKind::False,
        3 => JvKind::True,
        4 => JvKind::Number,
        5 => JvKind::String,
        6 => JvKind::Array,
        7 => JvKind::Object,
        _ => JvKind::Invalid,
    }
}
/// Helper function to get the numeric value from a jv
fn jv_number_value(v: &Jv) -> f64 {
    // Interpret u field as f64 bits when kind is Number
    if jv_get_kind(v) == JvKind::Number {
        f64::from_bits(v.u)
    } else {
        0.0
    }
}
/// Create a number Jv
pub fn jv_number(val: f64) -> Jv {
    Jv {
        kind_flags: JvKind::Number as u8,
        pad_: 0,
        offset: 0,
        size: 0,
        u: val.to_bits(),
    }
}
/// Create a string Jv
pub fn jv_string(s: &str) -> Jv {
    crate::jv::jv_string(s)
}
fn jv_string_fmt(fmt: &str, args: &[&str]) -> Jv {
    let mut result = fmt.to_string();
    for arg in args {
        result = result.replacen("%s", arg, 1);
    }
    Jv::string(&result)
}
/// Get string value from jv
/// Note: Strings in this representation store length in size field
/// This function delegates to the actual implementation in jv.rs
pub fn jv_string_value(v: &Jv) -> &str {
    crate::jv::jv_string_value(v)
}
/// Create a null jv
pub fn jv_null() -> Jv {
    Jv::null()
}
fn jv_invalid_with_msg(msg: Jv) -> Jv {
    Jv {
        kind_flags: JvKind::Invalid as u8,
        pad_: 0,
        offset: 0,
        size: msg.size,
        u: msg.u,
    }
}
/// Get array length - takes reference
pub fn jv_array_length(v: &Jv) -> i32 {
    // Array length is stored in the size field
    if jv_get_kind(v) == JvKind::Array {
        v.size
    } else {
        0
    }
}
/// Get array element
/// Note: This is a simplified implementation - returns null
/// Get array element at index
pub fn jv_array_get(v: &Jv, idx: i32) -> Jv {
    crate::jv::jv_array_get(v.copy(), idx)
}
/// Check if jv is valid
pub fn jv_is_valid(v: &Jv) -> bool {
    v.is_valid()
}
/// Creates a type error with the given message
pub fn type_error(input: Jv, msg: &str) -> Jv {
    let kind_str = match jv_get_kind(&input) {
        JvKind::Null => "null",
        JvKind::False | JvKind::True => "boolean",
        JvKind::Number => "number",
        JvKind::String => "string",
        JvKind::Array => "array",
        JvKind::Object => "object",
        JvKind::Invalid => "invalid",
    };
    let error_msg = format!("{} ({:?}) {}", kind_str, input, msg);
    jv_free(input);
    jv_invalid_with_msg(jv_string(&error_msg))
}
/// Create a type error with two values
pub fn type_error2(a: Jv, b: Jv, msg: &str) -> Jv {
    let kind_a = match jv_get_kind(&a) {
        JvKind::Null => "null",
        JvKind::False | JvKind::True => "boolean",
        JvKind::Number => "number",
        JvKind::String => "string",
        JvKind::Array => "array",
        JvKind::Object => "object",
        JvKind::Invalid => "invalid",
    };
    let kind_b = match jv_get_kind(&b) {
        JvKind::Null => "null",
        JvKind::False | JvKind::True => "boolean",
        JvKind::Number => "number",
        JvKind::String => "string",
        JvKind::Array => "array",
        JvKind::Object => "object",
        JvKind::Invalid => "invalid",
    };
    jv_free(a);
    jv_free(b);
    jv_invalid_with_msg(jv_string(&format!("{} and {} {}", kind_a, kind_b, msg)))
}
/// Return an error with a single value
pub fn ret_error(a: Jv, err: Jv) -> Jv {
    jv_free(a);
    // Pass the error Jv directly since invalid_with_msg takes a Jv
    Jv::invalid_with_msg(err)
}
/// Returns error with two bad values freed
#[inline]
pub fn ret_error2(bad1: Jv, bad2: Jv, msg: Jv) -> Jv {
    jv_free(bad1);
    jv_free(bad2);
    jv_invalid_with_msg(msg)
}
/// Convert tm struct to jv array
pub fn tm2jv(tm: &Tm) -> Jv {
    let mut arr = Jv::array();
    arr = jv_array_append(arr, jv_number(tm.tm_sec as f64));
    arr = jv_array_append(arr, jv_number(tm.tm_min as f64));
    arr = jv_array_append(arr, jv_number(tm.tm_hour as f64));
    arr = jv_array_append(arr, jv_number(tm.tm_mday as f64));
    arr = jv_array_append(arr, jv_number(tm.tm_mon as f64));
    arr = jv_array_append(arr, jv_number(tm.tm_year as f64));
    arr = jv_array_append(arr, jv_number(tm.tm_wday as f64));
    arr = jv_array_append(arr, jv_number(tm.tm_yday as f64));
    arr
}
/// Convert a Jv array to a Tm structure
fn jv2tm(a: &Jv, tm: &mut Tm) -> bool {
    if jv_get_kind(a) != JvKind::Array {
        return false;
    }
    if jv_array_length(a) < 6 {
        return false;
    }
    let sec = jv_array_get(a, 0);
    let min = jv_array_get(a, 1);
    let hour = jv_array_get(a, 2);
    let mday = jv_array_get(a, 3);
    let mon = jv_array_get(a, 4);
    let year = jv_array_get(a, 5);
    if jv_get_kind(&sec) != JvKind::Number || jv_get_kind(&min) != JvKind::Number
        || jv_get_kind(&hour) != JvKind::Number || jv_get_kind(&mday) != JvKind::Number
        || jv_get_kind(&mon) != JvKind::Number || jv_get_kind(&year) != JvKind::Number
    {
        return false;
    }
    tm.tm_sec = jv_number_value(&sec) as i32;
    tm.tm_min = jv_number_value(&min) as i32;
    tm.tm_hour = jv_number_value(&hour) as i32;
    tm.tm_mday = jv_number_value(&mday) as i32;
    tm.tm_mon = jv_number_value(&mon) as i32;
    tm.tm_year = jv_number_value(&year) as i32;
    if jv_array_length(a) > 6 {
        let wday = jv_array_get(a, 6);
        if jv_get_kind(&wday) == JvKind::Number {
            tm.tm_wday = jv_number_value(&wday) as i32;
        }
    }
    if jv_array_length(a) > 7 {
        let yday = jv_array_get(a, 7);
        if jv_get_kind(&yday) == JvKind::Number {
            tm.tm_yday = jv_number_value(&yday) as i32;
        }
    }
    true
}
/// Set the weekday field of a tm struct using Zeller's congruence
pub fn set_tm_wday(tm: &mut Tm) {
    let century = (1900 + tm.tm_year) / 100;
    let mut year = (1900 + tm.tm_year) % 100;
    if tm.tm_mon < 2 {
        year -= 1;
    }
    let mut mon = tm.tm_mon - 1;
    if mon < 1 {
        mon += 12;
    }
    let mut wday = (tm.tm_mday + (2.6 * mon as f64 - 0.2).floor() as i32 + year
        + (year as f64 / 4.0).floor() as i32 + (century as f64 / 4.0).floor() as i32
        - 2 * century) % 7;
    if wday < 0 {
        wday += 7;
    }
    tm.tm_wday = wday;
}
/// Set the tm_yday field based on other fields
pub fn set_tm_yday(tm: &mut Tm) {
    const DAYS_BEFORE_MONTH: [i32; 12] = [
        0, 31, 59, 90, 120, 151, 181, 212, 243, 273, 304, 334,
    ];
    let mut mon = tm.tm_mon;
    let year = 1900 + tm.tm_year;
    let is_leap = (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0);
    let leap_day = if tm.tm_mon > 1 && is_leap { 1 } else { 0 };
    if mon < 0 {
        mon = -mon;
    }
    if mon > 11 {
        mon %= 12;
    }
    let yday = DAYS_BEFORE_MONTH[mon as usize] + leap_day + tm.tm_mday - 1;
    assert!(
        yday == tm.tm_yday || tm.tm_yday == 367,
        "yday == tm->tm_yday || tm->tm_yday == 367"
    );
    tm.tm_yday = yday;
}
/// f_gmtime - Convert Unix timestamp to broken-down time (UTC)
pub fn f_gmtime<T>(_jq: &mut JqState<T>, a: Jv) -> Jv {
    if jv_get_kind(&a) != JvKind::Number {
        return ret_error(a, jv_string("gmtime() requires numeric inputs"));
    }
    let fsecs = jv_number_value(&a);
    let secs = fsecs as i64;
    jv_free(a);
    match gmtime_r(secs) {
        Some(tm) => {
            let mut arr = tm2jv(&tm);
            let frac = fsecs - fsecs.floor();
            let current_sec = jv_number_value(&jv_array_get(&jv_copy(&arr), 5));
            arr = jv_array_set(arr, 5, jv_number(current_sec + frac));
            arr
        }
        None => {
            jv_invalid_with_msg(
                jv_string("error converting number of seconds since epoch to datetime"),
            )
        }
    }
}
/// binop_lesseq implementation
pub fn binop_lesseq(a: Jv, b: Jv) -> Jv {
    let cmp = jv_cmp(a, b);
    if cmp <= 0 {
        Jv {
            kind_flags: JvKind::True as u8,
            pad_: 0,
            offset: 0,
            size: 0,
            u: 0,
        }
    } else {
        Jv {
            kind_flags: JvKind::False as u8,
            pad_: 0,
            offset: 0,
            size: 0,
            u: 0,
        }
    }
}
/// Scale floating point number by power of 2 (scalb)
pub fn f_scalb<T>(_jq: &mut JqState<T>, input: Jv, a: Jv, b: Jv) -> Jv {
    jv_free(input);
    if jv_get_kind(&a) != JvKind::Number {
        jv_free(b);
        return type_error(a, "number required");
    }
    if jv_get_kind(&b) != JvKind::Number {
        jv_free(a);
        return type_error(b, "number required");
    }
    let a_val = jv_number_value(&a);
    let b_val = jv_number_value(&b);
    let result = a_val * 2.0_f64.powf(b_val);
    let ret = jv_number(result);
    jv_free(a);
    jv_free(b);
    ret
}
/// Parse time string according to format
pub fn f_strptime<T>(_jq: &mut JqState<T>, a: Jv, b: Jv) -> Jv {
    if jv_get_kind(&a) != JvKind::String || jv_get_kind(&b) != JvKind::String {
        return ret_error2(
            a,
            b,
            jv_string("strptime/1 requires string inputs and arguments"),
        );
    }
    let mut tm = Tm::default();
    tm.tm_wday = 8;
    tm.tm_yday = 367;
    let input = jv_string_value(&a);
    let fmt = jv_string_value(&b);
    let parse_result = strptime_simple(input, fmt, &mut tm);
    match parse_result {
        Some(remaining) => {
            jv_free(b);
            set_tm_wday(&mut tm);
            set_tm_yday(&mut tm);
            let mut r = tm2jv(&tm);
            if !remaining.is_empty() {
                r = jv_array_append(r, jv_string(remaining));
            }
            jv_free(a);
            r
        }
        None => {
            ret_error2(a.clone(), b.clone(), jv_string_fmt("date does not match format", &[input, fmt]))
        }
    }
}
/// strptime implementation - parses date string according to format
fn strptime_simple<'a>(input: &'a str, fmt: &str, tm: &mut Tm) -> Option<&'a str> {
    let input_bytes = input.as_bytes();
    let fmt_bytes = fmt.as_bytes();
    let mut input_pos = 0;
    let mut fmt_pos = 0;

    while fmt_pos < fmt_bytes.len() {
        if fmt_bytes[fmt_pos] == b'%' {
            fmt_pos += 1;
            if fmt_pos >= fmt_bytes.len() {
                return None;
            }
            let spec = fmt_bytes[fmt_pos];
            fmt_pos += 1;

            match spec {
                b'Y' => {
                    // 4-digit year
                    let (val, consumed) = parse_int(&input_bytes[input_pos..], 4)?;
                    tm.tm_year = val - 1900;
                    input_pos += consumed;
                }
                b'y' => {
                    // 2-digit year
                    let (val, consumed) = parse_int(&input_bytes[input_pos..], 2)?;
                    tm.tm_year = if val < 69 { val + 100 } else { val };
                    input_pos += consumed;
                }
                b'm' => {
                    // Month 01-12
                    let (val, consumed) = parse_int(&input_bytes[input_pos..], 2)?;
                    if val < 1 || val > 12 { return None; }
                    tm.tm_mon = val - 1;
                    input_pos += consumed;
                }
                b'd' => {
                    // Day of month 01-31
                    let (val, consumed) = parse_int(&input_bytes[input_pos..], 2)?;
                    if val < 1 || val > 31 { return None; }
                    tm.tm_mday = val;
                    input_pos += consumed;
                }
                b'H' => {
                    // Hour 00-23
                    let (val, consumed) = parse_int(&input_bytes[input_pos..], 2)?;
                    if val > 23 { return None; }
                    tm.tm_hour = val;
                    input_pos += consumed;
                }
                b'M' => {
                    // Minute 00-59
                    let (val, consumed) = parse_int(&input_bytes[input_pos..], 2)?;
                    if val > 59 { return None; }
                    tm.tm_min = val;
                    input_pos += consumed;
                }
                b'S' => {
                    // Second 00-61 (60,61 for leap seconds)
                    let (val, consumed) = parse_int(&input_bytes[input_pos..], 2)?;
                    if val > 61 { return None; }
                    tm.tm_sec = val;
                    input_pos += consumed;
                }
                b'j' => {
                    // Day of year 001-366
                    let (val, consumed) = parse_int(&input_bytes[input_pos..], 3)?;
                    if val < 1 || val > 366 { return None; }
                    tm.tm_yday = val - 1;
                    input_pos += consumed;
                }
                b'w' => {
                    // Weekday 0-6
                    let (val, consumed) = parse_int(&input_bytes[input_pos..], 1)?;
                    if val > 6 { return None; }
                    tm.tm_wday = val;
                    input_pos += consumed;
                }
                b'n' | b't' => {
                    // Whitespace
                    while input_pos < input_bytes.len() &&
                          (input_bytes[input_pos] == b' ' || input_bytes[input_pos] == b'\t') {
                        input_pos += 1;
                    }
                }
                b'%' => {
                    // Literal %
                    if input_pos >= input_bytes.len() || input_bytes[input_pos] != b'%' {
                        return None;
                    }
                    input_pos += 1;
                }
                b'Z' => {
                    // Timezone name - skip alphabetic chars
                    while input_pos < input_bytes.len() && input_bytes[input_pos].is_ascii_alphabetic() {
                        input_pos += 1;
                    }
                }
                b'z' => {
                    // Timezone offset (+/-HHMM)
                    if input_pos >= input_bytes.len() { return None; }
                    if input_bytes[input_pos] == b'+' || input_bytes[input_pos] == b'-' {
                        input_pos += 1;
                        let (_, consumed) = parse_int(&input_bytes[input_pos..], 4)?;
                        input_pos += consumed;
                    }
                }
                _ => {
                    // Unknown specifier - try to skip
                }
            }
        } else if fmt_bytes[fmt_pos].is_ascii_whitespace() {
            // Format has whitespace - skip whitespace in input
            fmt_pos += 1;
            while input_pos < input_bytes.len() && input_bytes[input_pos].is_ascii_whitespace() {
                input_pos += 1;
            }
        } else {
            // Literal character - must match
            if input_pos >= input_bytes.len() || input_bytes[input_pos] != fmt_bytes[fmt_pos] {
                return None;
            }
            input_pos += 1;
            fmt_pos += 1;
        }
    }

    Some(&input[input_pos..])
}

/// Parse an integer from bytes, returning (value, bytes_consumed)
fn parse_int(bytes: &[u8], max_digits: usize) -> Option<(i32, usize)> {
    let mut val: i32 = 0;
    let mut consumed = 0;

    // Skip leading whitespace
    while consumed < bytes.len() && bytes[consumed].is_ascii_whitespace() {
        consumed += 1;
    }

    let start = consumed;
    while consumed < bytes.len() && consumed - start < max_digits && bytes[consumed].is_ascii_digit() {
        val = val * 10 + (bytes[consumed] - b'0') as i32;
        consumed += 1;
    }

    if consumed == start {
        return None;
    }

    Some((val, consumed))
}
/// Find minimum by key
pub fn f_min_by_impl<T>(_jq: &mut JqState<T>, x: Jv, y: Jv) -> Jv {
    minmax_by(x, y, 1)
}
/// Bessel function of second kind, order 0
pub fn f_y0<T>(_jq: &mut JqState<T>, input: Jv) -> Jv {
    if jv_get_kind(&input) != JvKind::Number {
        return type_error(input, "number required");
    }
    let x = jv_number_value(&input);
    let result = bessel_y0(x);
    let ret = jv_number(result);
    jv_free(input);
    ret
}
/// Bessel Y0 function approximation
fn bessel_y0(x: f64) -> f64 {
    if x <= 0.0 {
        return f64::NEG_INFINITY;
    }
    let j0 = bessel_j0(x);
    let euler_gamma = 0.5772156649015329;
    if x < 8.0 {
        let y = x * x;
        let result = (2.0 / std::f64::consts::PI) * ((x / 2.0).ln() + euler_gamma) * j0
            + (0.3674669052 + y * (-0.21098139e-1 + y * (0.35619938e-2)))
                / (1.0 + y * (0.15384e-2));
        result
    } else {
        let z = 8.0 / x;
        let y = z * z;
        let xx = x - 0.785398164;
        let p0 = 1.0 + y * (-0.1098628627e-2 + y * 0.2734510407e-4);
        let q0 = -0.1562499995e-1 + y * (0.1430488765e-3);
        (0.636619772 / x).sqrt() * (xx.sin() * p0 + z * xx.cos() * q0)
    }
}
/// Bessel J0 function approximation
fn bessel_j0(x: f64) -> f64 {
    let ax = x.abs();
    if ax < 8.0 {
        let y = x * x;
        let result = (57568490574.0
            + y
                * (-13362590354.0
                    + y
                        * (651619640.7
                            + y
                                * (-11214424.18 + y * (77392.33017 + y * (-184.9052456))))))
            / (57568490411.0
                + y
                    * (1029532985.0
                        + y
                            * (9494680.718
                                + y * (59272.64853 + y * (267.8532712 + y * 1.0)))));
        result
    } else {
        let z = 8.0 / ax;
        let y = z * z;
        let xx = ax - 0.785398164;
        let p0 = 1.0
            + y
                * (-0.1098628627e-2
                    + y
                        * (0.2734510407e-4
                            + y * (-0.2073370639e-5 + y * 0.2093887211e-6)));
        let q0 = -0.1562499995e-1
            + y
                * (0.1430488765e-3
                    + y
                        * (-0.6911147651e-5
                            + y * (0.7621095161e-6 + y * (-0.934945152e-7))));
        (0.636619772 / ax).sqrt() * (xx.cos() * p0 - z * xx.sin() * q0)
    }
}
/// Format time according to format string
pub fn f_strftime<T>(jq: &mut JqState<T>, a: Jv, b: Jv) -> Jv {
    let a = if jv_get_kind(&a) == JvKind::Number {
        let result = f_gmtime(jq, a);
        if !jv_is_valid(&result) {
            jv_free(b);
            return result;
        }
        result
    } else if jv_get_kind(&a) != JvKind::Array {
        return ret_error2(a, b, jv_string("strftime/1 requires parsed datetime inputs"));
    } else {
        a
    };
    if jv_get_kind(&b) != JvKind::String {
        return ret_error2(a, b, jv_string("strftime/1 requires a string format"));
    }
    let mut tm = Tm::default();
    if !jv2tm(&a, &mut tm) {
        return ret_error(b, jv_string("strftime/1 requires parsed datetime inputs"));
    }
    let fmt = jv_string_value(&b);
    let result = strftime_simple(fmt, &tm);
    jv_free(b);
    match result {
        Some(formatted) => jv_string(&formatted),
        None => jv_invalid_with_msg(jv_string("strftime/1: unknown system failure")),
    }
}
/// strftime implementation - formats time according to format string
fn strftime_simple(fmt: &str, tm: &Tm) -> Option<String> {
    static WEEKDAY_ABBREV: [&str; 7] = ["Sun", "Mon", "Tue", "Wed", "Thu", "Fri", "Sat"];
    static WEEKDAY_FULL: [&str; 7] = ["Sunday", "Monday", "Tuesday", "Wednesday", "Thursday", "Friday", "Saturday"];
    static MONTH_ABBREV: [&str; 12] = ["Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec"];
    static MONTH_FULL: [&str; 12] = ["January", "February", "March", "April", "May", "June", "July", "August", "September", "October", "November", "December"];

    let mut result = String::with_capacity(fmt.len() + 100);
    let mut chars = fmt.chars().peekable();
    while let Some(c) = chars.next() {
        if c == '%' {
            match chars.next() {
                // Date
                Some('Y') => result.push_str(&format!("{:04}", tm.tm_year + 1900)),
                Some('y') => result.push_str(&format!("{:02}", (tm.tm_year + 1900) % 100)),
                Some('C') => result.push_str(&format!("{:02}", (tm.tm_year + 1900) / 100)),
                Some('m') => result.push_str(&format!("{:02}", tm.tm_mon + 1)),
                Some('d') => result.push_str(&format!("{:02}", tm.tm_mday)),
                Some('e') => result.push_str(&format!("{:2}", tm.tm_mday)),
                Some('j') => result.push_str(&format!("{:03}", tm.tm_yday + 1)),
                // Time
                Some('H') => result.push_str(&format!("{:02}", tm.tm_hour)),
                Some('I') => result.push_str(&format!("{:02}", if tm.tm_hour == 0 { 12 } else if tm.tm_hour > 12 { tm.tm_hour - 12 } else { tm.tm_hour })),
                Some('k') => result.push_str(&format!("{:2}", tm.tm_hour)),
                Some('l') => result.push_str(&format!("{:2}", if tm.tm_hour == 0 { 12 } else if tm.tm_hour > 12 { tm.tm_hour - 12 } else { tm.tm_hour })),
                Some('M') => result.push_str(&format!("{:02}", tm.tm_min)),
                Some('S') => result.push_str(&format!("{:02}", tm.tm_sec)),
                Some('p') => result.push_str(if tm.tm_hour < 12 { "AM" } else { "PM" }),
                Some('P') => result.push_str(if tm.tm_hour < 12 { "am" } else { "pm" }),
                // Weekday
                Some('w') => result.push_str(&format!("{}", tm.tm_wday)),
                Some('u') => result.push_str(&format!("{}", if tm.tm_wday == 0 { 7 } else { tm.tm_wday })),
                Some('a') => result.push_str(WEEKDAY_ABBREV.get(tm.tm_wday as usize).unwrap_or(&"???")),
                Some('A') => result.push_str(WEEKDAY_FULL.get(tm.tm_wday as usize).unwrap_or(&"???")),
                // Month name
                Some('b') | Some('h') => result.push_str(MONTH_ABBREV.get(tm.tm_mon as usize).unwrap_or(&"???")),
                Some('B') => result.push_str(MONTH_FULL.get(tm.tm_mon as usize).unwrap_or(&"???")),
                // Composite
                Some('D') => result.push_str(&format!("{:02}/{:02}/{:02}", tm.tm_mon + 1, tm.tm_mday, (tm.tm_year + 1900) % 100)),
                Some('F') => result.push_str(&format!("{:04}-{:02}-{:02}", tm.tm_year + 1900, tm.tm_mon + 1, tm.tm_mday)),
                Some('T') => result.push_str(&format!("{:02}:{:02}:{:02}", tm.tm_hour, tm.tm_min, tm.tm_sec)),
                Some('R') => result.push_str(&format!("{:02}:{:02}", tm.tm_hour, tm.tm_min)),
                // Special
                Some('n') => result.push('\n'),
                Some('t') => result.push('\t'),
                Some('%') => result.push('%'),
                Some('Z') => result.push_str("UTC"), // Simplified - always UTC
                Some('z') => result.push_str("+0000"), // Simplified - always UTC
                Some('s') => {
                    // Seconds since epoch - use existing timegm function
                    let epoch_seconds = timegm(tm);
                    result.push_str(&format!("{}", epoch_seconds));
                }
                Some(other) => {
                    // Unknown specifier - pass through
                    result.push('%');
                    result.push(other);
                }
                None => result.push('%'),
            }
        } else {
            result.push(c);
        }
    }
    Some(result)
}

/// Extract exponent of floating point number (logb)
pub fn f_logb<T>(_jq: &mut JqState<T>, input: Jv) -> Jv {
    if jv_get_kind(&input) != JvKind::Number {
        return type_error(input, "number required");
    }
    let x = jv_number_value(&input);
    let result = if x == 0.0 {
        f64::NEG_INFINITY
    } else if x.is_infinite() {
        f64::INFINITY
    } else if x.is_nan() {
        f64::NAN
    } else {
        x.abs().log2().floor()
    };
    let ret = jv_number(result);
    jv_free(input);
    ret
}
/// Find min or max by key
pub fn minmax_by(values: Jv, keys: Jv, is_min: i32) -> Jv {
    if jv_get_kind(&values) != JvKind::Array {
        return type_error2(values, keys, "cannot be iterated over");
    }
    if jv_get_kind(&keys) != JvKind::Array {
        return type_error2(values, keys, "cannot be iterated over");
    }
    let values_len = jv_array_length(&values);
    let keys_len = jv_array_length(&keys);
    if values_len != keys_len {
        return type_error2(values, keys, "have wrong length");
    }
    if values_len == 0 {
        jv_free(values);
        jv_free(keys);
        return jv_null();
    }
    let mut ret = jv_array_get(&values, 0);
    let mut retkey = jv_array_get(&keys, 0);
    for i in 1..values_len {
        let item = jv_array_get(&keys, i);
        let cmp = jv_cmp(jv_copy(&item), jv_copy(&retkey));
        if (cmp < 0) == (is_min == 1) {
            jv_free(retkey);
            retkey = item;
            jv_free(ret);
            ret = jv_array_get(&values, i);
        } else {
            jv_free(item);
        }
    }
    jv_free(values);
    jv_free(keys);
    jv_free(retkey);
    ret
}
/// Hyperbolic tangent
pub fn f_tanh<T>(_jq: &mut JqState<T>, input: Jv) -> Jv {
    if jv_get_kind(&input) != JvKind::Number {
        return type_error(input, "number required");
    }
    let x = jv_number_value(&input);
    let result = x.tanh();
    let ret = jv_number(result);
    jv_free(input);
    ret
}
/// Less than or equal comparison
pub fn f_lesseq<T>(_jq: &mut JqState<T>, input: Jv, a: Jv, b: Jv) -> Jv {
    jv_free(input);
    binop_lesseq(a, b)
}
/// Check if object/array has key
pub fn f_has<T>(_jq: &mut JqState<T>, a: Jv, b: Jv) -> Jv {
    jv_has(a, b)
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_jv_number() {
        let v = jv_number(42.0);
        assert_eq!(jv_get_kind(& v), JvKind::Number);
        assert!((jv_number_value(& v) - 42.0).abs() < f64::EPSILON);
    }
    #[test]
    fn test_jv_string() {
        let v = jv_string("hello");
        assert_eq!(jv_get_kind(& v), JvKind::String);
        assert_eq!(jv_string_value(& v), "hello");
    }
    #[test]
    fn test_binop_minus_numbers() {
        let a = jv_number(10.0);
        let b = jv_number(3.0);
        let result = binop_minus(a, b);
        assert_eq!(jv_get_kind(& result), JvKind::Number);
        assert!((jv_number_value(& result) - 7.0).abs() < f64::EPSILON);
    }
    #[test]
    fn test_f_floor() {
        let mut jq: JqState<()> = JqState {
            _phantom: std::marker::PhantomData,
            ..Default::default()
        };
        let input = jv_number(3.7);
        let result = f_floor(&mut jq, input);
        assert_eq!(jv_get_kind(& result), JvKind::Number);
        assert!((jv_number_value(& result) - 3.0).abs() < f64::EPSILON);
    }
    #[test]
    fn test_f_asinh() {
        let mut jq: JqState<()> = JqState {
            _phantom: std::marker::PhantomData,
            ..Default::default()
        };
        let input = jv_number(0.0);
        let result = f_asinh(&mut jq, input);
        assert_eq!(jv_get_kind(& result), JvKind::Number);
        assert!((jv_number_value(& result) - 0.0).abs() < f64::EPSILON);
    }
}
/// Helper function to create jv true
fn jv_true() -> Jv {
    Jv::bool_val(true)
}
/// Create a false Jv
pub fn jv_false() -> Jv {
    jv_bool(false)
}
fn jv_array() -> Jv {
    Jv {
        kind_flags: JvKind::Array as u8,
        pad_: 0,
        offset: 0,
        size: 0,
        u: 0,
    }
}
/// Get the name of a jv kind
fn jv_kind_name(kind: JvKind) -> &'static str {
    match kind {
        JvKind::Invalid => "invalid",
        JvKind::Null => "null",
        JvKind::False => "boolean",
        JvKind::True => "boolean",
        JvKind::Number => "number",
        JvKind::String => "string",
        JvKind::Array => "array",
        JvKind::Object => "object",
    }
}
/// Compute the exponential of the input number
pub fn f_exp<T>(_jq: &mut JqState<T>, input: Jv) -> Jv {
    if jv_get_kind(&input) != JvKind::Number {
        return type_error(input, "number required");
    }
    let ret = jv_number(jv_number_value(&input).exp());
    jv_free(input);
    ret
}
/// Get the program origin from jq state
pub fn f_get_prog_origin<T>(jq: &mut JqState<T>, input: Jv) -> Jv {
    jv_free(input);
    jq_get_prog_origin(jq)
}
/// Binary comparison: less than
pub fn binop_less(a: Jv, b: Jv) -> Jv {
    let cmp_result = jv_cmp(a, b);
    if cmp_result < 0 { jv_true() } else { jv_false() }
}
/// Compare two values and return true if a < b
pub fn f_less<T>(_jq: &mut JqState<T>, input: Jv, a: Jv, b: Jv) -> Jv {
    jv_free(input);
    binop_less(a, b)
}
/// Binary subtraction operator
pub fn binop_minus(a: Jv, b: Jv) -> Jv {
    let kind_a = jv_get_kind(&a);
    let kind_b = jv_get_kind(&b);
    if kind_a == JvKind::Number && kind_b == JvKind::Number {
        let r = jv_number(jv_number_value(&a) - jv_number_value(&b));
        jv_free(a);
        jv_free(b);
        r
    } else if kind_a == JvKind::Array && kind_b == JvKind::Array {
        let mut out = jv_array();
        let len_a = jv_array_length(&a);
        let len_b = jv_array_length(&b);
        for i in 0..len_a {
            let x = jv_array_get(&a, i);
            let mut include = true;
            for j in 0..len_b {
                let y = jv_array_get(&b, j);
                if jv_equal(jv_copy(&x), y) {
                    include = false;
                    break;
                }
            }
            if include {
                out = jv_array_append(out, jv_copy(&x));
            }
            jv_free(x);
        }
        jv_free(a);
        jv_free(b);
        out
    } else {
        type_error2(a, b, "cannot be subtracted")
    }
}
/// Subtract b from a
pub fn f_minus<T>(_jq: &mut JqState<T>, input: Jv, a: Jv, b: Jv) -> Jv {
    jv_free(input);
    binop_minus(a, b)
}
/// Load x * 2^exp
pub fn f_ldexp<T>(_jq: &mut JqState<T>, input: Jv, a: Jv, b: Jv) -> Jv {
    jv_free(input);
    if jv_get_kind(&a) != JvKind::Number {
        jv_free(b);
        return type_error(a, "number required");
    }
    if jv_get_kind(&b) != JvKind::Number {
        jv_free(a);
        return type_error(b, "number required");
    }
    let mantissa = jv_number_value(&a);
    let exponent = jv_number_value(&b) as i32;
    let ret = jv_number(ldexp_impl(mantissa, exponent));
    jv_free(a);
    jv_free(b);
    ret
}
/// Extract significand from a floating point number
fn __jq_significand(x: f64) -> f64 {
    if x == 0.0 || x.is_nan() || x.is_infinite() {
        return x;
    }
    let bits = x.to_bits();
    let sign = bits >> 63;
    let mantissa = bits & 0x000F_FFFF_FFFF_FFFF;
    let new_bits = (sign << 63) | (1023u64 << 52) | mantissa;
    f64::from_bits(new_bits)
}
pub fn f_isnan<T>(_jq: &mut JqState<T>, input: Jv) -> Jv {
    if input.get_kind() != JvKind::Number {
        return type_error(input, "number required");
    }
    let val = input.number_value();
    jv_free(input);
    jv_bool(val.is_nan())
}
const CMP_OP_LESS: CmpOp = CmpOp::Less;
const CMP_OP_GREATER: CmpOp = CmpOp::Greater;
const CMP_OP_LESSEQ: CmpOp = CmpOp::LessEq;
const CMP_OP_GREATEREQ: CmpOp = CmpOp::GreaterEq;
/// Get string length in bytes
pub fn jv_string_length_bytes(v: Jv) -> usize {
    // String length is stored in size field
    let len = if jv_get_kind(&v) == JvKind::String { v.size as usize } else { 0 };
    jv_free(v);
    len
}
fn jv_number_with_literal(s: &str) -> Jv {
    // Parse the string as a number
    match s.parse::<f64>() {
        Ok(n) => jv_number(n),
        Err(_) => jv_null(),
    }
}
fn jv_parse_sized(s: &str, len: i32) -> Jv {
    crate::jv_parse::jv_parse_sized(s, len)
}
/// Create a boolean Jv
pub fn jv_bool(val: bool) -> Jv {
    Jv {
        kind_flags: if val { JvKind::True as u8 } else { JvKind::False as u8 },
        pad_: 0,
        offset: 0,
        size: 0,
        u: 0,
    }
}
// Note: jv_cmp is imported from jv_aux
pub fn order_cmp(a: &Jv, b: &Jv) -> i32 {
    let kind_a = jv_get_kind(a) as i32;
    let kind_b = jv_get_kind(b) as i32;
    if kind_a != kind_b {
        return kind_a - kind_b;
    }
    // For numbers, compare using jv_number_value
    if jv_get_kind(a) == JvKind::Number {
        let na = jv_number_value(a);
        let nb = jv_number_value(b);
        if na < nb { -1 } else if na > nb { 1 } else { 0 }
    } else {
        0
    }
}
/// Square root function
pub fn f_sqrt<T>(_jq: &JqState<T>, input: Jv) -> Jv {
    if jv_get_kind(&input) != JvKind::Number {
        return type_error(input, "number required");
    }
    let value = jv_number_value(&input);
    let ret = jv_number(value.sqrt());
    jv_free(input);
    ret
}
/// Ceiling function
pub fn f_ceil<T>(_jq: &JqState<T>, input: Jv) -> Jv {
    if jv_get_kind(&input) != JvKind::Number {
        return type_error(input, "number required");
    }
    let value = jv_number_value(&input);
    let ret = jv_number(value.ceil());
    jv_free(input);
    ret
}
/// Convert to number
pub fn f_tonumber<T>(_jq: &JqState<T>, input: Jv) -> Jv {
    if jv_get_kind(&input) == JvKind::Number {
        return input;
    }
    if jv_get_kind(&input) == JvKind::String {
        let s = jv_string_value(&input);
        if !s.is_empty() {
            let number = jv_number_with_literal(s);
            if jv_get_kind(&number) == JvKind::Invalid {
                return type_error(input, "cannot be parsed as a number");
            }
            jv_free(input);
            return number;
        }
    }
    type_error(input, "cannot be parsed as a number")
}
/// Greater than or equal comparison
pub fn binop_greatereq(a: Jv, b: Jv) -> Jv {
    let result = match (jv_get_kind(&a), jv_get_kind(&b)) {
        (JvKind::Number, JvKind::Number) => jv_number_value(&a) >= jv_number_value(&b),
        _ => false,
    };
    jv_free(a);
    jv_free(b);
    Jv {
        kind_flags: if result { JvKind::True } else { JvKind::False } as u8,
        pad_: 0,
        offset: 0,
        size: 0,
        u: 0,
    }
}
/// Arctangent function
pub fn f_atan<T>(_jq: &JqState<T>, input: Jv) -> Jv {
    if jv_get_kind(&input) != JvKind::Number {
        return type_error(input, "number required");
    }
    let value = jv_number_value(&input);
    let ret = jv_number(value.atan());
    jv_free(input);
    ret
}
/// UTF-8 byte length of string
pub fn f_utf8bytelength<T>(_jq: &JqState<T>, input: Jv) -> Jv {
    if jv_get_kind(&input) != JvKind::String {
        return type_error(input, "only strings have UTF-8 byte length");
    }
    let len = jv_string_length_bytes(jv_copy(&input));
    jv_free(input);
    jv_number(len as f64)
}
/// Parse JSON string
pub fn f_json_parse<T>(_jq: &JqState<T>, input: Jv) -> Jv {
    if jv_get_kind(&input) != JvKind::String {
        return type_error(input, "only strings can be parsed");
    }
    let len = jv_string_length_bytes(jv_copy(&input)) as i32;
    let s = jv_string_value(&input);
    let res = jv_parse_sized(s, len);
    jv_free(input);
    res
}
/// Bessel function of first kind, order 1
pub fn f_j1<T>(_jq: &JqState<T>, input: Jv) -> Jv {
    if jv_get_kind(&input) != JvKind::Number {
        return type_error(input, "number required");
    }
    let value = jv_number_value(&input);
    let ret = jv_number(bessel_j1(value));
    jv_free(input);
    ret
}
/// Arcsine function
pub fn f_asin<T>(_jq: &JqState<T>, input: Jv) -> Jv {
    if jv_get_kind(&input) != JvKind::Number {
        return type_error(input, "number required");
    }
    let value = jv_number_value(&input);
    let ret = jv_number(value.asin());
    jv_free(input);
    ret
}
/// Bessel function of first kind, order 1
/// This is an approximation - for production use, consider using a specialized math library
fn bessel_j1(x: f64) -> f64 {
    if x.abs() < 8.0 {
        let y = x * x;
        let ans1 = x
            * (72362614232.0
                + y
                    * (-7895059235.0
                        + y
                            * (242396853.1
                                + y
                                    * (-2972611.439
                                        + y * (15704.48260 + y * (-30.16036606))))));
        let ans2 = 144725228442.0
            + y
                * (2300535178.0
                    + y
                        * (18583304.74
                            + y * (99447.43394 + y * (376.9991397 + y * 1.0))));
        ans1 / ans2
    } else {
        let ax = x.abs();
        let z = 8.0 / ax;
        let y = z * z;
        let xx = ax - 2.356194491;
        let ans1 = 1.0
            + y
                * (0.183105e-2
                    + y
                        * (-0.3516396496e-4
                            + y * (0.2457520174e-5 + y * (-0.240337019e-6))));
        let ans2 = 0.04687499995
            + y
                * (-0.2002690873e-3
                    + y * (0.8449199096e-5 + y * (-0.88228987e-6 + y * 0.105787412e-6)));
        let ans = (0.636619772 / ax).sqrt() * (xx.cos() * ans1 - z * xx.sin() * ans2);
        if x < 0.0 { -ans } else { ans }
    }
}
fn jv_invalid() -> Jv {
    Jv::invalid()
}
/// Set array element
pub fn jv_array_set(v: Jv, idx: i32, val: Jv) -> Jv {
    crate::jv::jv_array_set(v, idx, val)
}
fn jv_contains(a: &Jv, b: &Jv) -> bool {
    crate::jv::jv_contains(a.copy(), b.copy()) != 0
}
/// Custom mktime implementation
fn my_mktime(tm: &Tm) -> Result<i64, i32> {
    let year = tm.tm_year as i64 + 1900;
    let mon = tm.tm_mon as i64;
    let mday = tm.tm_mday as i64;
    let hour = tm.tm_hour as i64;
    let min = tm.tm_min as i64;
    let sec = tm.tm_sec as i64;
    let mut days: i64 = 0;
    for y in 1970..year {
        days += if is_leap_year(y) { 366 } else { 365 };
    }
    let days_in_month = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
    for m in 0..mon {
        days += days_in_month[m as usize] as i64;
        if m == 1 && is_leap_year(year) {
            days += 1;
        }
    }
    days += mday - 1;
    let timestamp = days * 86400 + hour * 3600 + min * 60 + sec;
    Ok(timestamp)
}
fn is_leap_year(year: i64) -> bool {
    (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0)
}
/// Convert timestamp to broken-down time (UTC)
fn gmtime_r(secs: i64) -> Option<Tm> {
    let mut tm = Tm::default();
    let mut remaining = secs;
    let mut year = 1970i64;
    loop {
        let days_in_year = if is_leap_year(year) { 366 } else { 365 };
        let secs_in_year = days_in_year * 86400;
        if remaining < secs_in_year {
            break;
        }
        remaining -= secs_in_year;
        year += 1;
    }
    tm.tm_year = (year - 1900) as i32;
    let mut yday = (remaining / 86400) as i32;
    tm.tm_yday = yday;
    remaining %= 86400;
    let days_in_month = if is_leap_year(year) {
        [31, 29, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31]
    } else {
        [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31]
    };
    let mut mon = 0;
    while mon < 12 && yday >= days_in_month[mon] {
        yday -= days_in_month[mon];
        mon += 1;
    }
    tm.tm_mon = mon as i32;
    tm.tm_mday = yday + 1;
    tm.tm_hour = (remaining / 3600) as i32;
    remaining %= 3600;
    tm.tm_min = (remaining / 60) as i32;
    tm.tm_sec = (remaining % 60) as i32;
    let total_days = secs / 86400;
    tm.tm_wday = ((total_days + 4) % 7) as i32;
    Some(tm)
}
/// f_mktime - Convert broken-down time to Unix timestamp
pub fn f_mktime<T>(_jq: &mut JqState<T>, a: Jv) -> Jv {
    if jv_get_kind(&a) != JvKind::Array {
        return ret_error(a, jv_string("mktime requires array inputs"));
    }
    if jv_array_length(&jv_copy(&a)) < 6 {
        return ret_error(a, jv_string("mktime requires parsed datetime inputs"));
    }
    let mut tm = Tm::default();
    if !jv2tm(&a, &mut tm) {
        jv_free(a);
        return jv_invalid_with_msg(jv_string("mktime requires parsed datetime inputs"));
    }
    jv_free(a);
    match my_mktime(&tm) {
        Ok(t) => jv_number(t as f64),
        Err(-1) => jv_invalid_with_msg(jv_string("invalid gmtime representation")),
        Err(-2) => {
            jv_invalid_with_msg(jv_string("mktime not supported on this platform"))
        }
        Err(_) => jv_invalid_with_msg(jv_string("mktime error")),
    }
}
/// f_contains - Check if a contains b
pub fn f_contains<T>(_jq: &mut JqState<T>, a: Jv, b: Jv) -> Jv {
    if jv_get_kind(&a) == jv_get_kind(&b) {
        let result = jv_contains(&a, &b);
        jv_free(a);
        jv_free(b);
        jv_bool(result)
    } else {
        type_error2(a, b, "cannot have their containment checked")
    }
}
/// binop_mod - Modulo operation
pub fn binop_mod(a: Jv, b: Jv) -> Jv {
    if jv_get_kind(&a) == JvKind::Number && jv_get_kind(&b) == JvKind::Number {
        let na = jv_number_value(&a);
        let nb = jv_number_value(&b);
        if na.is_nan() || nb.is_nan() {
            jv_free(a);
            jv_free(b);
            return jv_number(f64::NAN);
        }
        let bi: i64 = if nb < i64::MIN as f64 {
            i64::MIN
        } else if nb > i64::MAX as f64 {
            i64::MAX
        } else {
            nb as i64
        };
        if bi == 0 {
            return type_error2(
                a,
                b,
                "cannot be divided (remainder) because the divisor is zero",
            );
        }
        let ai: i64 = if na < i64::MIN as f64 {
            i64::MIN
        } else if na > i64::MAX as f64 {
            i64::MAX
        } else {
            na as i64
        };
        let result = if bi == -1 { 0 } else { ai % bi };
        let r = jv_number(result as f64);
        jv_free(a);
        jv_free(b);
        r
    } else {
        type_error2(a, b, "cannot be divided (remainder)")
    }
}
/// f_sinh - Hyperbolic sine
pub fn f_sinh<T>(_jq: &mut JqState<T>, input: Jv) -> Jv {
    if jv_get_kind(&input) != JvKind::Number {
        return type_error(input, "number required");
    }
    let ret = jv_number(jv_number_value(&input).sinh());
    jv_free(input);
    ret
}
/// f_halt - Halt execution
pub fn f_halt<T>(jq: &mut JqState<T>, input: Jv) -> Jv {
    jv_free(input);
    jq_halt(jq, jv_invalid(), jv_invalid());
    jv_true()
}
/// f_fdim - Positive difference
pub fn f_fdim<T>(_jq: &mut JqState<T>, input: Jv, a: Jv, b: Jv) -> Jv {
    jv_free(input);
    if jv_get_kind(&a) != JvKind::Number {
        jv_free(b);
        return type_error(a, "number required");
    }
    if jv_get_kind(&b) != JvKind::Number {
        jv_free(a);
        return type_error(b, "number required");
    }
    let va = jv_number_value(&a);
    let vb = jv_number_value(&b);
    let ret = jv_number(if va > vb { va - vb } else { 0.0 });
    jv_free(a);
    jv_free(b);
    ret
}
/// f_setpath - Set a value at a path
pub fn f_setpath<T>(_jq: &mut JqState<T>, a: Jv, b: Jv, c: Jv) -> Jv {
    jv_setpath(a, b, c)
}
/// f_fmod - Floating-point modulo
pub fn f_fmod<T>(_jq: &mut JqState<T>, input: Jv, a: Jv, b: Jv) -> Jv {
    jv_free(input);
    if jv_get_kind(&a) != JvKind::Number {
        jv_free(b);
        return type_error(a, "number required");
    }
    if jv_get_kind(&b) != JvKind::Number {
        jv_free(a);
        return type_error(b, "number required");
    }
    let ret = jv_number(jv_number_value(&a) % jv_number_value(&b));
    jv_free(a);
    jv_free(b);
    ret
}
pub fn f_isnormal<T>(_jq: &mut JqState<T>, input: Jv) -> Jv {
    if input.get_kind() != JvKind::Number {
        return type_error(input, "number required");
    }
    let val = input.number_value();
    jv_free(input);
    let is_normal = val.is_normal();
    jv_bool(is_normal)
}
pub const BACKTRACK: i32 = 0;
pub const PATH_BEGIN: i32 = 1;
pub const PATH_END: i32 = 2;
pub const DUP: i32 = 3;
pub const STOREV: i32 = 4;
pub const LOADV: i32 = 5;
pub const RANGE: i32 = 6;
/// Arc cosine function
pub fn f_acos<T>(_jq: &mut JqState<T>, input: Jv) -> Jv {
    if jv_get_kind(&input) != JvKind::Number {
        return type_error(input, "number required");
    }
    let ret = jv_number(input.number_value().acos());
    jv_free(input);
    ret
}
/// Binary comparison returning greater result
pub fn binop_greater(a: Jv, b: Jv) -> Jv {
    let result = jv_cmp(jv_copy(&a), jv_copy(&b));
    jv_free(a);
    jv_free(b);
    jv_bool(result > 0)
}
/// Absolute value function
pub fn f_fabs<T>(_jq: &mut JqState<T>, input: Jv) -> Jv {
    if jv_get_kind(&input) != JvKind::Number {
        return type_error(input, "number required");
    }
    let ret = jv_number(input.number_value().abs());
    jv_free(input);
    ret
}
/// Maximum of two numbers
pub fn f_fmax<T>(_jq: &mut JqState<T>, input: Jv, a: Jv, b: Jv) -> Jv {
    jv_free(input);
    if jv_get_kind(&a) != JvKind::Number {
        jv_free(b);
        return type_error(a, "number required");
    }
    if jv_get_kind(&b) != JvKind::Number {
        jv_free(a);
        return type_error(b, "number required");
    }
    let ret = jv_number(a.number_value().max(b.number_value()));
    jv_free(a);
    jv_free(b);
    ret
}
/// Sort by implementation
pub fn f_sort_by_impl<T>(_jq: &mut JqState<T>, input: Jv, keys: Jv) -> Jv {
    if jv_get_kind(&input) == JvKind::Array && jv_get_kind(&keys) == JvKind::Array
        && jv_copy(&input).array_length() == jv_copy(&keys).array_length()
    {
        jv_sort(input, keys)
    } else {
        type_error2(input, keys, "cannot be sorted, as they are not both arrays")
    }
}
/// Check if string starts with another string
pub fn f_startswith<T>(_jq: &mut JqState<T>, a: Jv, b: Jv) -> Jv {
    if jv_get_kind(&a) != JvKind::String || jv_get_kind(&b) != JvKind::String {
        return ret_error2(a, b, jv_string("startswith() requires string inputs"));
    }
    let alen = jv_copy(&a).string_length_bytes();
    let blen = jv_copy(&b).string_length_bytes();
    let ret = if blen <= alen {
        let a_str = a.string_value().unwrap_or("");
        let b_str = b.string_value().unwrap_or("");
        if a_str.starts_with(b_str) { jv_true() } else { jv_false() }
    } else {
        jv_false()
    };
    jv_free(a);
    jv_free(b);
    ret
}
// Old bind_bytecoded_builtins removed - new version is in builtins_bind
/// Hyperbolic arc tangent function
pub fn f_atanh<T>(_jq: &mut JqState<T>, input: Jv) -> Jv {
    if jv_get_kind(&input) != JvKind::Number {
        return type_error(input, "number required");
    }
    let ret = jv_number(input.number_value().atanh());
    jv_free(input);
    ret
}
/// Helper to get current line from jq input (stub implementation)
fn jq_util_input_get_current_line<T>(_jq: &JqState<T>) -> Jv {
    Jv::null()
}
/// Division operation for jv values
/// Handles numeric division and string splitting
pub fn binop_divide(a: Jv, b: Jv) -> Jv {
    if jv_get_kind(&a) == JvKind::Number && jv_get_kind(&b) == JvKind::Number {
        if jv_number_value(&b) == 0.0 {
            return type_error2(a, b, "cannot be divided because the divisor is zero");
        }
        let r = jv_number(jv_number_value(&a) / jv_number_value(&b));
        jv_free(a);
        jv_free(b);
        r
    } else if jv_get_kind(&a) == JvKind::String && jv_get_kind(&b) == JvKind::String {
        jv_string_split(a, b)
    } else {
        type_error2(a, b, "cannot be divided")
    }
}
/// Negate a numeric value
pub fn f_negate<T>(_jq: &JqState<T>, input: Jv) -> Jv {
    if jv_get_kind(&input) != JvKind::Number {
        return type_error(input, "cannot be negated");
    }
    let ret = jv_number(-jv_number_value(&input));
    jv_free(input);
    ret
}
/// Get the current line number from input
pub fn f_current_line<T>(jq: &JqState<T>, a: Jv) -> Jv {
    jv_free(a);
    jq_util_input_get_current_line(jq)
}
/// Split a number into integer and fractional parts
pub fn f_modf<T>(_jq: &JqState<T>, input: Jv) -> Jv {
    if jv_get_kind(&input) != JvKind::Number {
        return type_error(input, "number required");
    }
    let num = jv_number_value(&input);
    let int_part = num.trunc();
    let frac_part = num.fract();
    let ret = jv_array_append(
        jv_array_append(jv_array(), jv_number(frac_part)),
        jv_number(int_part),
    );
    jv_free(input);
    ret
}
pub fn f_have_decnum<T>(_jq: &mut JqState<T>, input: Jv) -> Jv {
    jv_free(input);
    jv_bool(true)
}
/// Get keys from an object or array without sorting
pub fn f_keys_unsorted<T>(_jq: &JqState<T>, input: Jv) -> Jv {
    if jv_get_kind(&input) == JvKind::Object || jv_get_kind(&input) == JvKind::Array {
        jv_keys_unsorted(input)
    } else {
        type_error(input, "has no keys")
    }
}
/// Split a string by a separator
pub fn f_string_split<T>(_jq: &JqState<T>, a: Jv, b: Jv) -> Jv {
    if jv_get_kind(&a) != JvKind::String || jv_get_kind(&b) != JvKind::String {
        return ret_error2(a, b, jv_string("split input and separator must be strings"));
    }
    jv_string_split(a, b)
}
/// Sort an array
pub fn f_sort<T>(_jq: &JqState<T>, input: Jv) -> Jv {
    if jv_get_kind(&input) == JvKind::Array {
        let copy = jv_copy(&input);
        jv_sort(input, copy)
    } else {
        type_error(input, "cannot be sorted, as it is not an array")
    }
}
/// Halt execution with an error code
pub fn f_halt_error<T>(jq: &mut JqState<T>, input: Jv, a: Jv) -> Jv {
    if jv_get_kind(&a) != JvKind::Number {
        jv_free(a);
        return type_error(input, "halt_error/1: number required");
    }
    jq_halt(jq, a, input);
    jv_true()
}
/// Explode a string into an array of codepoints
pub fn f_string_explode<T>(_jq: &JqState<T>, a: Jv) -> Jv {
    if jv_get_kind(&a) != JvKind::String {
        return ret_error(a, jv_string("explode input must be a string"));
    }
    jv_string_explode(a)
}
/// Check if number is NaN
pub fn jvp_number_is_nan(v: &Jv) -> bool {
    if v.get_kind() == JvKind::Number { v.number_value().is_nan() } else { false }
}
/// Compute nearbyint of input
pub fn f_nearbyint<T>(_jq: &mut JqState<T>, input: Jv) -> Jv {
    if jv_get_kind(&input) != JvKind::Number {
        return type_error(input, "number required");
    }
    let ret = jv_number(jv_number_value(&input).round());
    jv_free(input);
    ret
}
/// Compute exp2 (2^x) of input
pub fn f_exp2<T>(_jq: &mut JqState<T>, input: Jv) -> Jv {
    if jv_get_kind(&input) != JvKind::Number {
        return type_error(input, "number required");
    }
    let ret = jv_number(jv_number_value(&input).exp2());
    jv_free(input);
    ret
}
/// Compute copysign - magnitude of a with sign of b
pub fn f_copysign<T>(_jq: &mut JqState<T>, input: Jv, a: Jv, b: Jv) -> Jv {
    jv_free(input);
    if jv_get_kind(&a) != JvKind::Number {
        jv_free(b);
        return type_error(a, "number required");
    }
    if jv_get_kind(&b) != JvKind::Number {
        jv_free(a);
        return type_error(b, "number required");
    }
    let ret = jv_number(jv_number_value(&a).copysign(jv_number_value(&b)));
    jv_free(a);
    jv_free(b);
    ret
}
/// Compute log2 of input
pub fn f_log2<T>(_jq: &mut JqState<T>, input: Jv) -> Jv {
    if jv_get_kind(&input) != JvKind::Number {
        return type_error(input, "number required");
    }
    let ret = jv_number(jv_number_value(&input).log2());
    jv_free(input);
    ret
}
/// Bessel function of the second kind (yn)
pub fn f_yn<T>(_jq: &mut JqState<T>, input: Jv, a: Jv, b: Jv) -> Jv {
    jv_free(input);
    if jv_get_kind(&a) != JvKind::Number {
        jv_free(b);
        return type_error(a, "number required");
    }
    if jv_get_kind(&b) != JvKind::Number {
        jv_free(a);
        return type_error(b, "number required");
    }
    let result = yn_impl(jv_number_value(&a) as i32, jv_number_value(&b));
    let ret = jv_number(result);
    jv_free(a);
    jv_free(b);
    ret
}
/// Simple Bessel Y_n approximation (placeholder)
fn bessel_yn(n: i32, x: f64) -> f64 {
    if x <= 0.0 {
        return f64::NEG_INFINITY;
    }
    f64::NAN
}
/// Compute expm1 (e^x - 1) of input
pub fn f_expm1<T>(_jq: &mut JqState<T>, input: Jv) -> Jv {
    if jv_get_kind(&input) != JvKind::Number {
        return type_error(input, "number required");
    }
    let ret = jv_number(jv_number_value(&input).exp_m1());
    jv_free(input);
    ret
}
/// Get the jq origin path
pub fn f_get_jq_origin<T>(jq: &mut JqState<T>, input: Jv) -> Jv {
    jv_free(input);
    jv_invalid()
}
/// Compute cube root of input
pub fn f_cbrt<T>(_jq: &mut JqState<T>, input: Jv) -> Jv {
    if jv_get_kind(&input) != JvKind::Number {
        return type_error(input, "number required");
    }
    let ret = jv_number(jv_number_value(&input).cbrt());
    jv_free(input);
    ret
}
const TRIM_LEFT: i32 = 1;
/// Trim whitespace from string based on operation flags
pub fn string_trim(a: Jv, op: i32) -> Jv {
    if a.get_kind() != JvKind::String {
        return ret_error(a, Jv::string("trim input must be a string"));
    }
    let s = a.string_value().unwrap_or("");
    let len = a.string_length_bytes() as usize;
    if len == 0 {
        return a;
    }
    let mut trim_start = 0usize;
    let mut trim_end = len;
    if (op & TRIM_LEFT) != 0 {
        let chars: Vec<char> = s.chars().collect();
        let mut pos = 0;
        for c in &chars {
            if !jvp_codepoint_is_whitespace(*c) {
                break;
            }
            pos += c.len_utf8();
        }
        trim_start = pos;
    }
    if (op & TRIM_RIGHT) != 0 && trim_end > trim_start {
        let substring = &s[trim_start..trim_end];
        let chars: Vec<char> = substring.chars().collect();
        let mut pos = trim_end;
        for c in chars.iter().rev() {
            if !jvp_codepoint_is_whitespace(*c) {
                break;
            }
            pos -= c.len_utf8();
            if pos <= trim_start {
                pos = trim_start;
                break;
            }
        }
        trim_end = pos;
    }
    if trim_start == 0 && trim_end == len {
        return a;
    }
    let trimmed = &s[trim_start..trim_end];
    let ts = Jv::string_sized(trimmed, (trim_end - trim_start) as i32);
    a.free();
    ts
}
pub fn f_significand<T>(_jq: &mut JqState<T>, input: Jv) -> Jv {
    if input.get_kind() != JvKind::Number {
        return type_error(input, "number required");
    }
    let val = input.number_value();
    jv_free(input);
    if val == 0.0 {
        return Jv::number(0.0);
    }
    let (mantissa, _exp, _sign) = decode_float(val);
    Jv::number(mantissa)
}
/// Bind bytecoded builtins (empty, not, path) - matches C's bind_bytecoded_builtins
fn bind_bytecoded_builtins(b: Block) -> Block {
    use crate::compile::{gen_noop, gen_op_simple, gen_function, gen_param, gen_call, gen_condbranch, gen_const, CompileOpcode};
    use crate::jv::{jv_true, jv_false};
    use crate::parser::block_join;

    let mut builtins = gen_noop();

    // empty = BACKTRACK
    let empty_body = gen_op_simple(CompileOpcode::BACKTRACK as u16);
    builtins = block_join(builtins, gen_function("empty", gen_noop(), empty_body));

    // not = if . then false else true end
    let not_body = gen_condbranch(gen_const(jv_false()), gen_const(jv_true()));
    builtins = block_join(builtins, gen_function("not", gen_noop(), not_body));

    // path(f) = PATH_BEGIN, f, PATH_END
    let path_body = block_join(
        block_join(
            gen_op_simple(CompileOpcode::PATH_BEGIN as u16),
            gen_call("arg", gen_noop())
        ),
        gen_op_simple(CompileOpcode::PATH_END as u16)
    );
    builtins = block_join(builtins, gen_function("path", gen_param("arg"), path_body));

    // range is defined in JQ_BUILTINS as jq code

    block_join(builtins, b)
}

/// Bind builtins to the jq state - matches C's builtins_bind in builtin.c
pub fn builtins_bind<T>(_jq: &mut JqState<T>, bb: Block) -> Block {
    use crate::compile::{gen_cbinding, block_bind_referenced, block_join, OP_IS_CALL_PSEUDO};
    use crate::types::Cfunction;
    use crate::parser::jq_parse_library;

    let debug = std::env::var("DEBUG_BUILTINS").is_ok();
    if debug { eprintln!("DEBUG builtins_bind: Step 1 - parsing JQ_BUILTINS"); }

    // Step 1: Parse JQ_BUILTINS (the embedded builtin.jq content)
    // This gives us jq-defined functions like map, select, add, unique, etc.
    let mut jq_builtins_block = Block::default();
    let mut locfile: Locfile<()> = Locfile::new("<builtin>", JQ_BUILTINS);
    let nerrors = jq_parse_library(&mut locfile, &mut jq_builtins_block);
    if debug { eprintln!("DEBUG builtins_bind: Step 1 done, nerrors={}", nerrors); }
    if nerrors != 0 {
        eprintln!("Warning: Failed to parse builtin.jq ({} errors)", nerrors);
        // Continue with empty block - native functions will still work
        jq_builtins_block = Block::default();
    }

    // Step 2: Add bytecoded builtins (empty, not, path)
    if debug { eprintln!("DEBUG builtins_bind: Step 2 - binding bytecoded builtins"); }
    let builtins = bind_bytecoded_builtins(jq_builtins_block);
    if debug { eprintln!("DEBUG builtins_bind: Step 2 done"); }

    // Step 3: Create the native function list - ONLY functions implemented in Rust
    // These mirror C jq's function_list in builtin.c
    // Note: Functions like add, unique, flatten, reverse, first, last, etc. are
    // jq-defined in JQ_BUILTINS, NOT native functions
    let function_list: Vec<Cfunction> = vec![
        // Core functions
        Cfunction { fptr: None, name: Some("keys".to_string()), nargs: 1 },
        Cfunction { fptr: None, name: Some("keys_unsorted".to_string()), nargs: 1 },
        Cfunction { fptr: None, name: Some("length".to_string()), nargs: 1 },
        Cfunction { fptr: None, name: Some("utf8bytelength".to_string()), nargs: 1 },
        Cfunction { fptr: None, name: Some("type".to_string()), nargs: 1 },
        Cfunction { fptr: None, name: Some("has".to_string()), nargs: 2 },
        Cfunction { fptr: None, name: Some("contains".to_string()), nargs: 2 },
        Cfunction { fptr: None, name: Some("sort".to_string()), nargs: 1 },
        Cfunction { fptr: None, name: Some("_sort_by_impl".to_string()), nargs: 2 },
        Cfunction { fptr: None, name: Some("_group_by_impl".to_string()), nargs: 2 },
        Cfunction { fptr: None, name: Some("min".to_string()), nargs: 1 },
        Cfunction { fptr: None, name: Some("max".to_string()), nargs: 1 },
        Cfunction { fptr: None, name: Some("_min_by_impl".to_string()), nargs: 2 },
        Cfunction { fptr: None, name: Some("_max_by_impl".to_string()), nargs: 2 },

        // String functions
        Cfunction { fptr: None, name: Some("startswith".to_string()), nargs: 2 },
        Cfunction { fptr: None, name: Some("endswith".to_string()), nargs: 2 },
        Cfunction { fptr: None, name: Some("split".to_string()), nargs: 2 },
        Cfunction { fptr: None, name: Some("explode".to_string()), nargs: 1 },
        Cfunction { fptr: None, name: Some("implode".to_string()), nargs: 1 },
        Cfunction { fptr: None, name: Some("_strindices".to_string()), nargs: 2 },
        Cfunction { fptr: None, name: Some("trim".to_string()), nargs: 1 },
        Cfunction { fptr: None, name: Some("ltrim".to_string()), nargs: 1 },
        Cfunction { fptr: None, name: Some("rtrim".to_string()), nargs: 1 },

        // Path functions
        Cfunction { fptr: None, name: Some("getpath".to_string()), nargs: 2 },
        Cfunction { fptr: None, name: Some("setpath".to_string()), nargs: 3 },
        Cfunction { fptr: None, name: Some("delpaths".to_string()), nargs: 2 },

        // Conversion functions
        Cfunction { fptr: None, name: Some("tonumber".to_string()), nargs: 1 },
        Cfunction { fptr: None, name: Some("tostring".to_string()), nargs: 1 },
        Cfunction { fptr: None, name: Some("tojson".to_string()), nargs: 1 },
        Cfunction { fptr: None, name: Some("fromjson".to_string()), nargs: 1 },

        // Type checking functions
        Cfunction { fptr: None, name: Some("isinfinite".to_string()), nargs: 1 },
        Cfunction { fptr: None, name: Some("isnan".to_string()), nargs: 1 },
        Cfunction { fptr: None, name: Some("isnormal".to_string()), nargs: 1 },
        Cfunction { fptr: None, name: Some("infinite".to_string()), nargs: 1 },
        Cfunction { fptr: None, name: Some("nan".to_string()), nargs: 1 },

        // Math functions (from libm)
        Cfunction { fptr: None, name: Some("floor".to_string()), nargs: 1 },
        Cfunction { fptr: None, name: Some("ceil".to_string()), nargs: 1 },
        Cfunction { fptr: None, name: Some("round".to_string()), nargs: 1 },
        Cfunction { fptr: None, name: Some("sqrt".to_string()), nargs: 1 },
        Cfunction { fptr: None, name: Some("fabs".to_string()), nargs: 1 },
        Cfunction { fptr: None, name: Some("sin".to_string()), nargs: 1 },
        Cfunction { fptr: None, name: Some("cos".to_string()), nargs: 1 },
        Cfunction { fptr: None, name: Some("tan".to_string()), nargs: 1 },
        Cfunction { fptr: None, name: Some("asin".to_string()), nargs: 1 },
        Cfunction { fptr: None, name: Some("acos".to_string()), nargs: 1 },
        Cfunction { fptr: None, name: Some("atan".to_string()), nargs: 1 },
        Cfunction { fptr: None, name: Some("log".to_string()), nargs: 1 },
        Cfunction { fptr: None, name: Some("log10".to_string()), nargs: 1 },
        Cfunction { fptr: None, name: Some("log2".to_string()), nargs: 1 },
        Cfunction { fptr: None, name: Some("exp".to_string()), nargs: 1 },
        Cfunction { fptr: None, name: Some("exp10".to_string()), nargs: 1 },
        Cfunction { fptr: None, name: Some("exp2".to_string()), nargs: 1 },
        Cfunction { fptr: None, name: Some("expm1".to_string()), nargs: 1 },
        Cfunction { fptr: None, name: Some("pow".to_string()), nargs: 2 },
        Cfunction { fptr: None, name: Some("atan2".to_string()), nargs: 2 },
        Cfunction { fptr: None, name: Some("fma".to_string()), nargs: 3 },

        // Error and control
        Cfunction { fptr: None, name: Some("error".to_string()), nargs: 1 },
        Cfunction { fptr: None, name: Some("format".to_string()), nargs: 2 },
        Cfunction { fptr: None, name: Some("halt".to_string()), nargs: 1 },
        Cfunction { fptr: None, name: Some("halt_error".to_string()), nargs: 2 },

        // Environment and I/O
        Cfunction { fptr: None, name: Some("env".to_string()), nargs: 1 },
        Cfunction { fptr: None, name: Some("now".to_string()), nargs: 1 },
        Cfunction { fptr: None, name: Some("debug".to_string()), nargs: 1 },
        Cfunction { fptr: None, name: Some("stderr".to_string()), nargs: 1 },
        Cfunction { fptr: None, name: Some("input".to_string()), nargs: 1 },
        Cfunction { fptr: None, name: Some("input_filename".to_string()), nargs: 1 },
        Cfunction { fptr: None, name: Some("input_line_number".to_string()), nargs: 1 },

        // Date/time functions
        Cfunction { fptr: None, name: Some("strptime".to_string()), nargs: 2 },
        Cfunction { fptr: None, name: Some("strftime".to_string()), nargs: 2 },
        Cfunction { fptr: None, name: Some("strflocaltime".to_string()), nargs: 2 },
        Cfunction { fptr: None, name: Some("mktime".to_string()), nargs: 1 },
        Cfunction { fptr: None, name: Some("gmtime".to_string()), nargs: 1 },
        Cfunction { fptr: None, name: Some("localtime".to_string()), nargs: 1 },

        // Regex (returns error if ONIGURUMA not available)
        Cfunction { fptr: None, name: Some("_match_impl".to_string()), nargs: 4 },

        // Module functions
        Cfunction { fptr: None, name: Some("modulemeta".to_string()), nargs: 1 },
        Cfunction { fptr: None, name: Some("get_search_list".to_string()), nargs: 1 },
        Cfunction { fptr: None, name: Some("get_prog_origin".to_string()), nargs: 1 },
        Cfunction { fptr: None, name: Some("get_jq_origin".to_string()), nargs: 1 },

        // Binary operators (internal)
        Cfunction { fptr: None, name: Some("_negate".to_string()), nargs: 1 },
        Cfunction { fptr: None, name: Some("_plus".to_string()), nargs: 3 },
        Cfunction { fptr: None, name: Some("_minus".to_string()), nargs: 3 },
        Cfunction { fptr: None, name: Some("_multiply".to_string()), nargs: 3 },
        Cfunction { fptr: None, name: Some("_divide".to_string()), nargs: 3 },
        Cfunction { fptr: None, name: Some("_mod".to_string()), nargs: 3 },
        Cfunction { fptr: None, name: Some("_equal".to_string()), nargs: 3 },
        Cfunction { fptr: None, name: Some("_notequal".to_string()), nargs: 3 },
        Cfunction { fptr: None, name: Some("_less".to_string()), nargs: 3 },
        Cfunction { fptr: None, name: Some("_greater".to_string()), nargs: 3 },
        Cfunction { fptr: None, name: Some("_lesseq".to_string()), nargs: 3 },
        Cfunction { fptr: None, name: Some("_greatereq".to_string()), nargs: 3 },
    ];

    let ncfunctions = function_list.len() as i32;
    if debug { eprintln!("DEBUG builtins_bind: Step 3 - gen_cbinding with {} cfunctions", ncfunctions); }
    let builtins_with_cfuncs = gen_cbinding(&function_list, ncfunctions, builtins);
    if debug { eprintln!("DEBUG builtins_bind: Step 3 done"); }

    // Step 4: Generate the builtins list
    if debug { eprintln!("DEBUG builtins_bind: Step 4 - gen_builtin_list"); }
    let final_builtins = gen_builtin_list(builtins_with_cfuncs);
    if debug { eprintln!("DEBUG builtins_bind: Step 4 done"); }

    // Step 5: Bind to the program block
    if debug { eprintln!("DEBUG builtins_bind: Step 5 - block_bind_referenced"); }
    let result = block_bind_referenced(final_builtins, bb, OP_IS_CALL_PSEUDO);
    if debug { eprintln!("DEBUG builtins_bind: Step 5 done"); }
    result
}
/// Returns the next representable floating-point value after a towards b
pub fn f_nextafter<T>(_jq: &mut JqState<T>, input: Jv, a: Jv, b: Jv) -> Jv {
    jv_free(input);
    if a.get_kind() != JvKind::Number {
        jv_free(b);
        return type_error(a, "number required");
    }
    if b.get_kind() != JvKind::Number {
        jv_free(a);
        return type_error(b, "number required");
    }
    let a_val = a.number_value();
    let b_val = b.number_value();
    let result = if a_val == b_val {
        a_val
    } else if a_val.is_nan() || b_val.is_nan() {
        f64::NAN
    } else {
        let bits = a_val.to_bits();
        let next_bits = if (b_val > a_val) == (a_val >= 0.0) {
            bits.wrapping_add(1)
        } else {
            bits.wrapping_sub(1)
        };
        f64::from_bits(next_bits)
    };
    let ret = Jv::number(result);
    jv_free(a);
    jv_free(b);
    ret
}
/// Returns the type name of the input value
pub fn f_type<T>(_jq: &mut JqState<T>, input: Jv) -> Jv {
    let out = Jv::string(jv_kind_name(input.get_kind()));
    jv_free(input);
    out
}
/// Trims whitespace from the left side of a string
pub fn f_string_ltrim<T>(_jq: &mut JqState<T>, a: Jv) -> Jv {
    string_trim(a, TRIM_LEFT)
}
/// Fused multiply-add: (a * b) + c
pub fn f_fma<T>(_jq: &mut JqState<T>, input: Jv, a: Jv, b: Jv, c: Jv) -> Jv {
    jv_free(input);
    if a.get_kind() != JvKind::Number {
        jv_free(b);
        jv_free(c);
        return type_error(a, "number required");
    }
    if b.get_kind() != JvKind::Number {
        jv_free(a);
        jv_free(c);
        return type_error(b, "number required");
    }
    if c.get_kind() != JvKind::Number {
        jv_free(a);
        jv_free(b);
        return type_error(c, "number required");
    }
    let a_val = a.number_value();
    let b_val = b.number_value();
    let c_val = c.number_value();
    let ret = Jv::number(a_val.mul_add(b_val, c_val));
    jv_free(a);
    jv_free(b);
    jv_free(c);
    ret
}
/// Error function
pub fn f_erf<T>(_jq: &mut JqState<T>, input: Jv) -> Jv {
    if input.get_kind() != JvKind::Number {
        return type_error(input, "number required");
    }
    let x = input.number_value();
    let result = erf_approx(x);
    let ret = Jv::number(result);
    jv_free(input);
    ret
}
/// Approximation of the error function
fn erf_approx(x: f64) -> f64 {
    let a1 = 0.254829592;
    let a2 = -0.284496736;
    let a3 = 1.421413741;
    let a4 = -1.453152027;
    let a5 = 1.061405429;
    let p = 0.3275911;
    let sign = if x < 0.0 { -1.0 } else { 1.0 };
    let x = x.abs();
    let t = 1.0 / (1.0 + p * x);
    let y = 1.0 - (((((a5 * t + a4) * t) + a3) * t + a2) * t + a1) * t * (-x * x).exp();
    sign * y
}
/// Returns whether a > b
pub fn f_greater<T>(_jq: &mut JqState<T>, input: Jv, a: Jv, b: Jv) -> Jv {
    jv_free(input);
    binop_greater(a, b)
}
/// Returns the minimum value in an array
pub fn f_min<T>(_jq: &mut JqState<T>, x: Jv) -> Jv {
    minmax_by(jv_copy(&x), x, 1)
}
/// Groups array elements by their keys
pub fn f_group_by_impl<T>(_jq: &mut JqState<T>, input: Jv, keys: Jv) -> Jv {
    if input.get_kind() == JvKind::Array && keys.get_kind() == JvKind::Array
        && input.copy().array_length() == keys.copy().array_length()
    {
        jv_group(input, keys)
    } else {
        type_error2(input, keys, "cannot be sorted, as they are not both arrays")
    }
}
const TRIM_RIGHT: i32 = 2;
/// Compute remainder of two numbers
pub fn f_remainder<T>(_jq: &mut JqState<T>, input: Jv, a: Jv, b: Jv) -> Jv {
    jv_free(input);
    if a.get_kind() != JvKind::Number {
        jv_free(b);
        return type_error(a, "number required");
    }
    if b.get_kind() != JvKind::Number {
        jv_free(a);
        return type_error(b, "number required");
    }
    let a_val = a.number_value();
    let b_val = b.number_value();
    jv_free(a);
    jv_free(b);
    Jv::number(a_val % b_val)
}
/// Block list functions helper (placeholder)
fn block_list_funcs(_builtins: &Block, _include_private: i32) -> Jv {
    Jv::array()
}
/// Generate a builtin list block
pub fn gen_builtin_list(builtins: Block) -> Block {
    use crate::compile::{gen_noop, gen_function, gen_const};
    use crate::parser::block_join;
    let list = block_list_funcs(&builtins, 1).array_append(Jv::string("builtins/0"));
    block_join(builtins, gen_function("builtins", gen_noop(), gen_const(list)))
}
/// Load module metadata (placeholder)
fn load_module_meta<T>(_jq: &mut JqState<T>, _name: Jv) -> Jv {
    Jv::null()
}
/// Get module metadata
pub fn f_modulemeta<T>(jq: &mut JqState<T>, a: Jv) -> Jv {
    if a.get_kind() != JvKind::String {
        return ret_error(a, Jv::string("modulemeta input module name must be a string"));
    }
    load_module_meta(jq, a)
}
/// Truncate a number
pub fn f_trunc<T>(_jq: &mut JqState<T>, input: Jv) -> Jv {
    if input.get_kind() != JvKind::Number {
        return type_error(input, "number required");
    }
    let val = input.number_value();
    jv_free(input);
    Jv::number(val.trunc())
}
/// Divide two values
pub fn f_divide<T>(_jq: &mut JqState<T>, input: Jv, a: Jv, b: Jv) -> Jv {
    jv_free(input);
    binop_divide(a, b)
}
/// Trim whitespace from both sides of a string
pub fn f_string_trim<T>(_jq: &mut JqState<T>, a: Jv) -> Jv {
    string_trim(a, TRIM_LEFT | TRIM_RIGHT)
}
/// Debug function - calls debug callback with input
pub fn f_debug<T>(jq: &mut JqState<T>, input: Jv) -> Jv {
    if let Some(cb) = jq.debug_cb.as_ref() {
        if let Some(data) = jq.debug_cb_data.as_mut() {
            cb(data, input.copy());
        }
    }
    input
}
/// Match function - returns error since ONIGURUMA is not available
pub fn f_match<T>(
    _jq: &mut JqState<T>,
    input: Jv,
    regex: Jv,
    modifiers: Jv,
    testmode: Jv,
) -> Jv {
    jv_free(input);
    jv_free(regex);
    jv_free(modifiers);
    jv_free(testmode);
    Jv::invalid_with_msg(
        Jv::string(
            "jq was compiled without ONIGURUMA regex library. match/test/sub and related functions are not available.",
        ),
    )
}
fn decode_float(val: f64) -> (f64, i32, i32) {
    let bits = val.to_bits();
    let sign = if (bits >> 63) != 0 { -1 } else { 1 };
    let exp = ((bits >> 52) & 0x7FF) as i32;
    let mantissa = if exp == 0 {
        (bits & 0xFFFFFFFFFFFFF) << 1
    } else {
        (bits & 0xFFFFFFFFFFFFF) | 0x10000000000000
    };
    let actual_exp = exp - 1023 - 52;
    let mantissa_f = mantissa as f64 * 2.0_f64.powi(actual_exp);
    (mantissa_f.abs(), exp - 1023, sign)
}
pub fn f_isinfinite<T>(_jq: &mut JqState<T>, input: Jv) -> Jv {
    if input.get_kind() != JvKind::Number {
        return type_error(input, "number required");
    }
    let val = input.number_value();
    jv_free(input);
    jv_bool(val.is_infinite())
}
/// Return positive infinity
pub fn f_infinite<T>(_jq: &mut JqState<T>, input: Jv) -> Jv {
    jv_free(input);
    Jv::number(f64::INFINITY)
}
/// Returns NaN as a jv number
pub fn f_nan<T>(_jq: &mut JqState<T>, input: Jv) -> Jv {
    drop(input);
    Jv::number(f64::NAN)
}
/// Returns current Unix timestamp with microsecond precision
pub fn f_now<T>(_jq: &mut JqState<T>, a: Jv) -> Jv {
    jv_free(a);
    match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(duration) => {
            let secs = duration.as_secs() as f64;
            let micros = duration.subsec_micros() as f64;
            jv_number(secs + micros / 1_000_000.0)
        }
        Err(_) => jv_number(0.0),
    }
}
/// Create an error from input
pub fn f_error<T>(_jq: &mut JqState<T>, input: Jv) -> Jv {
    Jv::invalid_with_msg(input)
}
/// Reads the next input value from the input callback
pub fn f_input<T>(jq: &mut JqState<T>, input: Jv) -> Jv {
    drop(input);
    // Simplified: callbacks require complex type handling
    // In a full implementation, this would retrieve input from the input callback
    let _ = jq;
    Jv::invalid_with_msg(Jv::string("break"))
}
/// Escapes special characters in a string based on escape mappings
pub fn escape_string(input: Jv, escapings: &str) -> Jv {
    assert!(jv_get_kind(& input) == JvKind::String);
    let mut lookup: HashMap<char, &str> = HashMap::new();
    lookup.insert('\0', "\\0");
    let mut chars = escapings.chars().peekable();
    while let Some(ch) = chars.next() {
        if ch == '\0' {
            break;
        }
        let mut replacement = String::new();
        while let Some(&next) = chars.peek() {
            if next == '\0' {
                chars.next();
                break;
            }
            replacement.push(chars.next().unwrap());
        }
    }
    let mut ret = String::new();
    let s = jv_string_value(&input);
    let end = s.len();
    let mut codepoint = 0i32;
    let mut remaining = s;
    while let Some(next) = jvp_utf8_next(remaining, &s[end..], &mut codepoint) {
        if (codepoint as u32) < 128 {
            if let Some(escape) = lookup.get(&(codepoint as u8 as char)) {
                ret.push_str(escape);
            } else {
                ret.push(codepoint as u8 as char);
            }
        } else {
            let char_len = remaining.len() - next.len();
            ret.push_str(&remaining[..char_len]);
        }
        remaining = next;
    }
    jv_free(input);
    jv_string(&ret)
}
const BASE64_ENCODE_TABLE: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
const BASE64_INVALID_ENTRY: u8 = 0xFF;
fn base64_decode_table() -> [u8; 256] {
    let mut table = [BASE64_INVALID_ENTRY; 256];
    for (i, &ch) in BASE64_ENCODE_TABLE.iter().enumerate() {
        table[ch as usize] = i as u8;
    }
    table
}
fn jv_string_append_str(v: Jv, s: &str) -> Jv {
    crate::jv::jv_string_append_str(v, s)
}
fn jv_string_append_buf(v: Jv, buf: &str, len: usize) -> Jv {
    crate::jv::jv_string_append_buf(v, buf.as_bytes(), len as i32)
}
fn jv_string_concat(a: Jv, b: Jv) -> Jv {
    crate::jv::jv_string_concat(a, b)
}
fn jv_string_sized(data: &str, len: usize) -> Jv {
    crate::jv::jv_string_sized(data, len)
}
fn jv_array_concat(a: Jv, b: Jv) -> Jv {
    crate::jv::jv_array_concat(a, b)
}
fn jv_object_merge(a: Jv, b: Jv) -> Jv {
    crate::jv::jv_object_merge(a, b)
}
/// Dump jv to string
pub fn jv_dump_string(input: Jv, flags: i32) -> Jv {
    crate::jv_print::jv_dump_string(input, flags)
}
fn jvp_utf8_next<'a>(
    start: &'a str,
    end: &'a str,
    codepoint: &mut i32,
) -> Option<&'a str> {
    if start.is_empty() || start.as_ptr() >= end.as_ptr() {
        return None;
    }
    let mut chars = start.chars();
    if let Some(c) = chars.next() {
        *codepoint = c as i32;
        Some(chars.as_str())
    } else {
        None
    }
}
/// Computes log base 10 of a number
pub fn f_log10(_jq: &mut JqState, input: Jv) -> Jv {
    if jv_get_kind(&input) != JvKind::Number {
        return type_error(input, "number required");
    }
    let value = jv_number_value(&input);
    let ret = jv_number(value.log10());
    jv_free(input);
    ret
}
/// Computes tangent of a number
pub fn f_tan(_jq: &mut JqState, input: Jv) -> Jv {
    if jv_get_kind(&input) != JvKind::Number {
        return type_error(input, "number required");
    }
    let value = jv_number_value(&input);
    let ret = jv_number(value.tan());
    jv_free(input);
    ret
}
/// Bessel function of the first kind, order 0
pub fn f_j0(_jq: &mut JqState, input: Jv) -> Jv {
    if jv_get_kind(&input) != JvKind::Number {
        return type_error(input, "number required");
    }
    let value = jv_number_value(&input);
    let ret = jv_number(bessel_j0(value));
    jv_free(input);
    ret
}
/// Computes minimum of two floating point numbers
pub fn f_fmin(_jq: &mut JqState, input: Jv, a: Jv, b: Jv) -> Jv {
    jv_free(input);
    if jv_get_kind(&a) != JvKind::Number {
        jv_free(b);
        return type_error(a, "number required");
    }
    if jv_get_kind(&b) != JvKind::Number {
        jv_free(a);
        return type_error(b, "number required");
    }
    let va = jv_number_value(&a);
    let vb = jv_number_value(&b);
    let ret = jv_number(va.min(vb));
    jv_free(a);
    jv_free(b);
    ret
}
/// Converts string to string (identity for strings, dump for others)
pub fn f_tostring<T>(_jq: &mut JqState<T>, input: Jv) -> Jv {
    match jv_get_kind(&input) {
        JvKind::String => input,
        _ => jv_dump_string(input, 0),
    }
}
/// Formats input according to the specified format string
pub fn f_format(jq: &mut JqState, input: Jv, fmt: Jv) -> Jv {
    if jv_get_kind(&fmt) != JvKind::String {
        jv_free(input);
        return type_error(fmt, "is not a valid format");
    }
    let fmt_s = jv_string_value(&fmt);
    match fmt_s {
        "json" => {
            jv_free(fmt);
            jv_dump_string(input, 0)
        }
        "text" => {
            jv_free(fmt);
            f_tostring(jq, input)
        }
        "csv" | "tsv" => {
            let (msg, quotes, sep, escapings) = if fmt_s == "csv" {
                ("cannot be csv-formatted, only array", "\"", ",", "\"\"\"\0")
            } else {
                (
                    "cannot be tsv-formatted, only array",
                    "",
                    "\t",
                    "\t\\t\0\r\\r\0\n\\n\0\\\\\\\0",
                )
            };
            jv_free(fmt);
            if jv_get_kind(&input) != JvKind::Array {
                return type_error(input, msg);
            }
            let mut line = jv_string("");
            let len = jv_array_length(&input);
            for i in 0..len {
                let x = jv_array_get(&input, i);
                if i > 0 {
                    line = jv_string_append_str(line, sep);
                }
                match jv_get_kind(&x) {
                    JvKind::Null => {
                        jv_free(x);
                    }
                    JvKind::True | JvKind::False => {
                        line = jv_string_concat(line, jv_dump_string(x, 0));
                    }
                    JvKind::Number => {
                        let val = jv_number_value(&x);
                        if val.is_nan() {
                            jv_free(x);
                        } else {
                            line = jv_string_concat(line, jv_dump_string(x, 0));
                        }
                    }
                    JvKind::String => {
                        line = jv_string_append_str(line, quotes);
                        line = jv_string_concat(line, escape_string(x, escapings));
                        line = jv_string_append_str(line, quotes);
                    }
                    _ => {
                        jv_free(input);
                        jv_free(line);
                        return type_error(x, "is not valid in a csv row");
                    }
                }
            }
            jv_free(input);
            line
        }
        "html" => {
            jv_free(fmt);
            escape_string(
                f_tostring(jq, input),
                "&&amp;\0<&lt;\0>&gt;\0'&apos;\0\"&quot;\0",
            )
        }
        "uri" => {
            jv_free(fmt);
            let input = f_tostring(jq, input);
            let unreserved: Vec<bool> = (0..128)
                .map(|c| {
                    let ch = c as u8 as char;
                    ch.is_ascii_alphanumeric() || "-_.~".contains(ch)
                })
                .collect();
            let mut line = String::new();
            let s = jv_string_value(&input);
            for byte in s.bytes() {
                if (byte as usize) < 128 && unreserved[byte as usize] {
                    line.push(byte as char);
                } else {
                    line.push_str(&format!("%{:02X}", byte));
                }
            }
            jv_free(input);
            jv_string(&line)
        }
        "sh" => {
            jv_free(fmt);
            let input = if jv_get_kind(&input) != JvKind::Array {
                jv_array_set(jv_array(), 0, input)
            } else {
                input
            };
            let mut line = jv_string("");
            let len = jv_array_length(&input);
            for i in 0..len {
                let x = jv_array_get(&input, i);
                if i > 0 {
                    line = jv_string_append_str(line, " ");
                }
                match jv_get_kind(&x) {
                    JvKind::Null | JvKind::True | JvKind::False | JvKind::Number => {
                        line = jv_string_concat(line, jv_dump_string(x, 0));
                    }
                    JvKind::String => {
                        line = jv_string_append_str(line, "'");
                        line = jv_string_concat(line, escape_string(x, "''\\''\0"));
                        line = jv_string_append_str(line, "'");
                    }
                    _ => {
                        jv_free(input);
                        jv_free(line);
                        return type_error(x, "can not be escaped for shell");
                    }
                }
            }
            jv_free(input);
            line
        }
        "base64" => {
            jv_free(fmt);
            let input = f_tostring(jq, input);
            let mut result = String::new();
            let s = jv_string_value(&input);
            let data = s.as_bytes();
            let len = data.len();
            for i in (0..len).step_by(3) {
                let mut code: u32 = 0;
                let n = if len - i >= 3 { 3 } else { len - i };
                for j in 0..3 {
                    code <<= 8;
                    if j < n {
                        code |= data[i + j] as u32;
                    }
                }
                let mut buf = [0u8; 4];
                for j in 0..4 {
                    buf[j] = BASE64_ENCODE_TABLE[((code >> (18 - j * 6)) & 0x3f)
                        as usize];
                }
                if n < 3 {
                    buf[3] = b'=';
                }
                if n < 2 {
                    buf[2] = b'=';
                }
                result.push_str(std::str::from_utf8(&buf).unwrap());
            }
            jv_free(input);
            jv_string(&result)
        }
        "base64d" => {
            jv_free(fmt);
            let input = f_tostring(jq, input);
            let decode_table = base64_decode_table();
            let s = jv_string_value(&input);
            let data = s.as_bytes();
            let len = data.len();
            let decoded_len = (3 * len) / 4;
            let mut result = Vec::with_capacity(decoded_len);
            let mut input_bytes_read = 0;
            let mut code: u32 = 0;
            for i in 0..len {
                if data[i] == b'=' {
                    break;
                }
                let decoded = decode_table[data[i] as usize];
                if decoded == BASE64_INVALID_ENTRY {
                    return type_error(input, "is not valid base64 data");
                }
                code <<= 6;
                code |= decoded as u32;
                input_bytes_read += 1;
                if input_bytes_read == 4 {
                    result.push(((code >> 16) & 0xFF) as u8);
                    result.push(((code >> 8) & 0xFF) as u8);
                    result.push((code & 0xFF) as u8);
                    input_bytes_read = 0;
                    code = 0;
                }
            }
            match input_bytes_read {
                3 => {
                    result.push(((code >> 10) & 0xFF) as u8);
                    result.push(((code >> 2) & 0xFF) as u8);
                }
                2 => {
                    result.push(((code >> 4) & 0xFF) as u8);
                }
                1 => {
                    return type_error(input, "trailing base64 byte found");
                }
                _ => {}
            }
            jv_free(input);
            let result_str = String::from_utf8_lossy(&result);
            jv_string(&result_str)
        }
        _ => {
            jv_free(input);
            jv_invalid_with_msg(
                jv_string_concat(fmt, jv_string(" is not a valid format")),
            )
        }
    }
}
/// Returns the maximum element by comparison key
pub fn f_max_by_impl(_jq: &mut JqState, x: Jv, y: Jv) -> Jv {
    minmax_by(x, y, 0)
}
/// Binary plus operation - handles null, number, string, array, and object addition
pub fn binop_plus(a: Jv, b: Jv) -> Jv {
    match (jv_get_kind(&a), jv_get_kind(&b)) {
        (JvKind::Null, _) => {
            jv_free(a);
            b
        }
        (_, JvKind::Null) => {
            jv_free(b);
            a
        }
        (JvKind::Number, JvKind::Number) => {
            let va = jv_number_value(&a);
            let vb = jv_number_value(&b);
            let r = jv_number(va + vb);
            jv_free(a);
            jv_free(b);
            r
        }
        (JvKind::String, JvKind::String) => jv_string_concat(a, b),
        (JvKind::Array, JvKind::Array) => jv_array_concat(a, b),
        (JvKind::Object, JvKind::Object) => jv_object_merge(a, b),
        _ => type_error2(a, b, "cannot be added"),
    }
}
/// Greater than or equal builtin function
pub fn f_greatereq(_jq: &mut JqState, input: Jv, a: Jv, b: Jv) -> Jv {
    jv_free(input);
    binop_greatereq(a, b)
}
fn jvp_utf8_backtrack(s: &str, pos: usize) -> Option<usize> {
    if pos == 0 {
        return None;
    }
    let bytes = s.as_bytes();
    let mut i = pos - 1;
    while i > 0 && (bytes[i] & 0xC0) == 0x80 {
        i -= 1;
    }
    Some(i)
}
fn jvp_codepoint_is_whitespace(c: char) -> bool {
    c.is_whitespace()
}
/// Compute lgamma_r and return [result, sign] array
pub fn f_lgamma_r<T>(_jq: &mut JqState<T>, input: Jv) -> Jv {
    if input.get_kind() != JvKind::Number {
        return type_error(input, "number required");
    }
    let x = input.number_value();
    let result = lgamma_impl(x);
    let sign = if tgamma_impl(x) >= 0.0 { 1 } else { -1 };
    let ret = Jv::array()
        .array_append(Jv::number(result))
        .array_append(Jv::number(sign as f64));
    input.free();
    ret
}
/// Compute frexp and return [mantissa, exponent] array
pub fn f_frexp<T>(_jq: &mut JqState<T>, input: Jv) -> Jv {
    if input.get_kind() != JvKind::Number {
        return type_error(input, "number required");
    }
    let x = input.number_value();
    let (mantissa, exponent) = frexp_impl(x);
    let ret = Jv::array()
        .array_append(Jv::number(mantissa))
        .array_append(Jv::number(exponent as f64));
    input.free();
    ret
}
/// Get all environment variables as an object
pub fn f_env<T>(_jq: &mut JqState<T>, input: Jv) -> Jv {
    input.free();
    let mut env_obj = Jv::object();
    for (key, value) in env::vars() {
        env_obj = env_obj.object_set(Jv::string(&key), Jv::string(&value));
    }
    env_obj
}
/// Binary operator: not equal
pub fn binop_notequal(a: Jv, b: Jv) -> Jv {
    jv_bool(!jv_equal(a, b))
}
/// Compute complementary error function
pub fn f_erfc<T>(_jq: &mut JqState<T>, input: Jv) -> Jv {
    if input.get_kind() != JvKind::Number {
        return type_error(input, "number required");
    }
    let ret = Jv::number(erfc_impl(input.number_value()));
    input.free();
    ret
}
/// Round to nearest integer
pub fn f_round<T>(_jq: &mut JqState<T>, input: Jv) -> Jv {
    if input.get_kind() != JvKind::Number {
        return type_error(input, "number required");
    }
    let ret = Jv::number(input.number_value().round());
    input.free();
    ret
}
/// Trim whitespace from right side of string
pub fn f_string_rtrim<T>(_jq: &mut JqState<T>, a: Jv) -> Jv {
    string_trim(a, TRIM_RIGHT)
}
fn jv_object_length(v: Jv) -> i32 {
    v.size
}
fn jv_string_length_codepoints(v: Jv) -> i32 {
    v.size
}
fn jv_string_indexes(a: Jv, b: Jv) -> Jv {
    crate::jv::jv_string_indexes(a, b)
}
/// Get the length of a jv value
pub fn f_length<T>(_jq: &mut JqState<T>, input: Jv) -> Jv {
    match jv_get_kind(&input) {
        JvKind::Array => Jv::number(jv_array_length(&input) as f64),
        JvKind::Object => Jv::number(jv_object_length(input) as f64),
        JvKind::String => Jv::number(jv_string_length_codepoints(input) as f64),
        JvKind::Number => {
            let val = jv_number_value(&input);
            let r = Jv::number(val.abs());
            jv_free(input);
            r
        }
        JvKind::Null => {
            jv_free(input);
            Jv::number(0.0)
        }
        _ => type_error(input, "has no length"),
    }
}
/// Multiply two jv values
pub fn f_multiply<T>(_jq: &mut JqState<T>, input: Jv, a: Jv, b: Jv) -> Jv {
    jv_free(input);
    binop_multiply(a, b)
}
/// Multiplies two jv values
/// - Numbers: arithmetic multiplication
/// - String * Number: string repetition
/// - Object * Object: recursive merge
pub fn binop_multiply(a: Jv, b: Jv) -> Jv {
    let ak = a.get_kind();
    let bk = b.get_kind();
    if ak == JvKind::Number && bk == JvKind::Number {
        let result = Jv::number(a.number_value() * b.number_value());
        drop(a);
        drop(b);
        return result;
    }
    if (ak == JvKind::String && bk == JvKind::Number)
        || (ak == JvKind::Number && bk == JvKind::String)
    {
        let (str_val, num_val) = if ak == JvKind::Number { (b, a) } else { (a, b) };
        let d = num_val.number_value();
        let res = if d < 0.0 || d.is_nan() {
            Jv::null()
        } else {
            let n = d as usize;
            let str_bytes = str_val.string_value().unwrap_or("");
            let alen = str_bytes.len();
            let mut result = String::with_capacity(alen * n);
            for _ in 0..n {
                result.push_str(str_bytes);
            }
            Jv::string(&result)
        };
        drop(str_val);
        drop(num_val);
        return res;
    }
    if ak == JvKind::Object && bk == JvKind::Object {
        return crate::jv::jv_object_merge_recursive(a, b);
    }
    type_error2(a, b, "cannot be multiplied")
}
/// Get the search list from jq state
pub fn f_get_search_list<T>(_jq: &mut JqState<T>, input: Jv) -> Jv {
    jv_free(input);
    jq_get_lib_dirs(_jq)
}
/// Get a value at a path
pub fn f_getpath<T>(jq: &mut JqState<T>, a: Jv, b: Jv) -> Jv {
    let a_copy = jv_copy(&a);
    let b_copy = jv_copy(&b);
    let result = jv_getpath(a_copy, b_copy);
    _jq_path_append(jq, a, b, result)
}
/// Compute gamma function
pub fn f_gamma<T>(_jq: &mut JqState<T>, input: Jv) -> Jv {
    if jv_get_kind(&input) != JvKind::Number {
        return type_error(input, "number required");
    }
    let val = jv_number_value(&input);
    let ret = Jv::number(tgamma(val));
    jv_free(input);
    ret
}
/// Gamma function implementation
fn tgamma(x: f64) -> f64 {
    if x <= 0.0 && x.floor() == x {
        return f64::NAN;
    }
    if x > 0.0 && x.floor() == x && x <= 20.0 {
        let n = x as i64 - 1;
        let mut result = 1.0;
        for i in 2..=n {
            result *= i as f64;
        }
        return result;
    }
    let g = 7.0;
    let c = [
        0.99999999999980993,
        676.5203681218851,
        -1259.1392167224028,
        771.32342877765313,
        -176.61502916214059,
        12.507343278686905,
        -0.13857109526572012,
        9.9843695780195716e-6,
        1.5056327351493116e-7,
    ];
    let x = if x < 0.5 {
        PI / (PI * x).sin() / tgamma(1.0 - x)
    } else {
        let x = x - 1.0;
        let mut a = c[0];
        for i in 1..9 {
            a += c[i] / (x + i as f64);
        }
        let t = x + g + 0.5;
        (2.0 * PI).sqrt() * t.powf(x + 0.5) * (-t).exp() * a
    };
    x
}
/// Compute 10^x
pub fn f_exp10<T>(_jq: &mut JqState<T>, input: Jv) -> Jv {
    if jv_get_kind(&input) != JvKind::Number {
        return type_error(input, "number required");
    }
    let val = jv_number_value(&input);
    let ret = Jv::number(10.0_f64.powf(val));
    jv_free(input);
    ret
}
/// Find maximum value in array
pub fn f_max<T>(_jq: &mut JqState<T>, x: Jv) -> Jv {
    let x_copy = jv_copy(&x);
    minmax_by(x, x_copy, 0)
}
/// Compute log-gamma function
pub fn f_lgamma<T>(_jq: &mut JqState<T>, input: Jv) -> Jv {
    if jv_get_kind(&input) != JvKind::Number {
        return type_error(input, "number required");
    }
    let val = jv_number_value(&input);
    let ret = Jv::number(lgamma(val));
    jv_free(input);
    ret
}
/// Log-gamma function implementation
fn lgamma(x: f64) -> f64 {
    if x <= 0.0 {
        return f64::INFINITY;
    }
    if x < 12.0 {
        return tgamma(x).abs().ln();
    }
    let c = [1.0 / 12.0, -1.0 / 360.0, 1.0 / 1260.0, -1.0 / 1680.0];
    let mut r = (2.0 * PI / x).sqrt().ln() + (x - 0.5) * x.ln() - x;
    let mut xn = x;
    for coef in c.iter() {
        r += coef / xn;
        xn *= x * x;
    }
    r
}
/// Find indexes of substring in string
pub fn f_string_indexes<T>(_jq: &mut JqState<T>, a: Jv, b: Jv) -> Jv {
    jv_string_indexes(a, b)
}
/// Returns true if a != b
pub fn f_notequal<T>(_jq: &mut JqState<T>, input: Jv, a: Jv, b: Jv) -> Jv {
    drop(input);
    binop_notequal(a, b)
}
/// Returns a + b
pub fn f_plus<T>(_jq: &mut JqState<T>, input: Jv, a: Jv, b: Jv) -> Jv {
    drop(input);
    binop_plus(a, b)
}
/// Returns atan2(a, b)
pub fn f_atan2<T>(_jq: &mut JqState<T>, input: Jv, a: Jv, b: Jv) -> Jv {
    drop(input);
    if a.get_kind() != JvKind::Number {
        drop(b);
        return type_error(a, "number required");
    }
    if b.get_kind() != JvKind::Number {
        drop(a);
        return type_error(b, "number required");
    }
    let ret = Jv::number(a.number_value().atan2(b.number_value()));
    drop(a);
    drop(b);
    ret
}
/// Returns acosh(input)
pub fn f_acosh<T>(_jq: &mut JqState<T>, input: Jv) -> Jv {
    if input.get_kind() != JvKind::Number {
        return type_error(input, "number required");
    }
    let ret = Jv::number(input.number_value().acosh());
    drop(input);
    ret
}
/// Returns pow(a, b)
pub fn f_pow<T>(_jq: &mut JqState<T>, input: Jv, a: Jv, b: Jv) -> Jv {
    drop(input);
    if a.get_kind() != JvKind::Number {
        drop(b);
        return type_error(a, "number required");
    }
    if b.get_kind() != JvKind::Number {
        drop(a);
        return type_error(b, "number required");
    }
    let ret = Jv::number(a.number_value().powf(b.number_value()));
    drop(a);
    drop(b);
    ret
}
/// Convert a tm struct to Unix timestamp (UTC)
/// This is a pure Rust implementation of timegm
fn timegm(tm: &Tm) -> i64 {
    const DAYS_IN_MONTH: [i32; 12] = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
    fn is_leap_year(year: i32) -> bool {
        (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0)
    }
    let year = tm.tm_year + 1900;
    let mut days: i64 = 0;
    for y in 1970..year {
        days += if is_leap_year(y) { 366 } else { 365 };
    }
    for y in (year..1970).rev() {
        days -= if is_leap_year(y) { 366 } else { 365 };
    }
    for m in 0..tm.tm_mon {
        days += DAYS_IN_MONTH[m as usize] as i64;
        if m == 1 && is_leap_year(year) {
            days += 1;
        }
    }
    days += (tm.tm_mday - 1) as i64;
    days * 86400 + (tm.tm_hour as i64) * 3600 + (tm.tm_min as i64) * 60
        + (tm.tm_sec as i64)
}
/// Get stderr callback from jq state
pub fn jq_get_stderr_cb<T>(jq: &JqState<T>) -> (Option<JqMsgCb<T>>, Option<&T>) {
    (jq.stderr_cb, jq.stderr_cb_data.as_ref().map(|b| b.as_ref()))
}
/// Get current filename from jq util input (placeholder)
pub fn jq_util_input_get_current_filename<T>(_jq: &JqState<T>) -> Jv {
    Jv::default()
}
/// Modulo operation for jq
pub fn f_mod<T>(_jq: &mut JqState<T>, input: Jv, a: Jv, b: Jv) -> Jv {
    jv_free(input);
    binop_mod(a, b)
}
/// Hyperbolic cosine function for jq
pub fn f_cosh<T>(_jq: &mut JqState<T>, input: Jv) -> Jv {
    if jv_get_kind(&input) != JvKind::Number {
        return type_error(input, "number required");
    }
    let ret = jv_number(jv_number_value(&input).cosh());
    jv_free(input);
    ret
}
/// Round to nearest integer function for jq
pub fn f_rint<T>(_jq: &mut JqState<T>, input: Jv) -> Jv {
    if jv_get_kind(&input) != JvKind::Number {
        return type_error(input, "number required");
    }
    let ret = jv_number(jv_number_value(&input).round());
    jv_free(input);
    ret
}
/// Stderr output function for jq
pub fn f_stderr<T>(jq: &mut JqState<T>, input: Jv) -> Jv {
    if let (Some(cb), Some(data)) = (jq.stderr_cb, jq.stderr_cb_data.as_mut()) {
        cb(data.as_mut(), jv_copy(&input));
    }
    input
}
/// Dump to string function for jq
pub fn f_dump<T>(_jq: &mut JqState<T>, input: Jv) -> Jv {
    jv_dump_string(input, 0)
}
/// Get current filename function for jq
pub fn f_current_filename<T>(jq: &mut JqState<T>, a: Jv) -> Jv {
    jv_free(a);
    let r = jq_util_input_get_current_filename(jq);
    if jv_is_valid(&r) {
        return r;
    }
    jv_free(r);
    jv_null()
}
/// Cosine function for jq
pub fn f_cos<T>(_jq: &mut JqState<T>, input: Jv) -> Jv {
    if jv_get_kind(&input) != JvKind::Number {
        return type_error(input, "number required");
    }
    let ret = jv_number(jv_number_value(&input).cos());
    jv_free(input);
    ret
}
/// Check if string ends with suffix
pub fn f_endswith<T>(_jq: &mut JqState<T>, a: Jv, b: Jv) -> Jv {
    if jv_get_kind(&a) != JvKind::String || jv_get_kind(&b) != JvKind::String {
        return ret_error2(a, b, jv_string("endswith() requires string inputs"));
    }
    let astr = jv_string_value(&a);
    let bstr = jv_string_value(&b);
    let alen = jv_string_length_bytes(jv_copy(&a));
    let blen = jv_string_length_bytes(jv_copy(&b));
    let ret = if alen < blen || !astr.ends_with(bstr) { jv_false() } else { jv_true() };
    jv_free(a);
    jv_free(b);
    ret
}
/// Natural log of (1 + x) function for jq
pub fn f_log1p<T>(_jq: &mut JqState<T>, input: Jv) -> Jv {
    if jv_get_kind(&input) != JvKind::Number {
        return type_error(input, "number required");
    }
    let ret = jv_number(jv_number_value(&input).ln_1p());
    jv_free(input);
    ret
}
/// Implode array of codepoints to string
pub fn f_string_implode<T>(_jq: &mut JqState<T>, a: Jv) -> Jv {
    if jv_get_kind(&a) != JvKind::Array {
        return ret_error(a, jv_string("implode input must be an array"));
    }
    let len = jv_array_length(&a);
    let mut result = String::new();
    for i in 0..len {
        let n = jv_array_get(&a, i);
        if jv_get_kind(&n) != JvKind::Number || jvp_number_is_nan(&n) {
            jv_free(a);
            return type_error(
                n,
                "can't be imploded, unicode codepoint needs to be numeric",
            );
        }
        let mut nv = jv_number_value(&n) as i32;
        jv_free(n);
        if nv < 0 || nv > 0x10FFFF || (nv >= 0xD800 && nv <= 0xDFFF) {
            nv = 0xFFFD;
        }
        if let Some(c) = char::from_u32(nv as u32) {
            result.push(c);
        }
    }
    jv_free(a);
    jv_string(&result)
}
/// Check if two jv values are equal
/// C: int jv_equal(jv a, jv b) in jv.c
pub fn jv_equal(a: Jv, b: Jv) -> bool {
    let ka = jv_get_kind(&a);
    let kb = jv_get_kind(&b);
    if ka != kb {
        return false;
    }
    match ka {
        JvKind::Null => true,
        JvKind::True => true,
        JvKind::False => true,
        JvKind::Number => {
            // C: jvp_number_equal
            let va = jv_number_value(&a);
            let vb = jv_number_value(&b);
            (va - vb).abs() < f64::EPSILON
        }
        JvKind::String => {
            // C: jvp_string_equal
            jv_string_value(&a) == jv_string_value(&b)
        }
        JvKind::Array => {
            // C: jvp_array_equal in jv.c lines 878-890
            let len_a = jv_array_length(&a);
            let len_b = jv_array_length(&b);
            if len_a != len_b {
                return false;
            }
            for i in 0..len_a {
                let elem_a = jv_array_get(&a, i);
                let elem_b = jv_array_get(&b, i);
                if !jv_equal(elem_a, elem_b) {
                    return false;
                }
            }
            true
        }
        JvKind::Object => {
            // C: jvp_object_equal in jv.c lines 1705-1718
            let keys_a = crate::jv_aux::jv_keys(jv_copy(&a));
            let keys_b = crate::jv_aux::jv_keys(jv_copy(&b));
            let len_a = jv_array_length(&keys_a);
            let len_b = jv_array_length(&keys_b);
            if len_a != len_b {
                jv_free(keys_a);
                jv_free(keys_b);
                return false;
            }
            // Check each key in a exists in b with equal value
            for i in 0..len_a {
                let key = jv_array_get(&keys_a, i);
                let val_a = crate::jv::jv_object_get(&a, jv_copy(&key));
                let val_b = crate::jv::jv_object_get(&b, jv_copy(&key));
                jv_free(key);
                if !jv_equal(val_a, val_b) {
                    jv_free(keys_a);
                    jv_free(keys_b);
                    return false;
                }
            }
            jv_free(keys_a);
            jv_free(keys_b);
            true
        }
        JvKind::Invalid => false,
    }
}
/// Format a jv value for display
fn format_jv(v: &Jv) -> String {
    match jv_get_kind(v) {
        JvKind::Null => "null".to_string(),
        JvKind::True => "true".to_string(),
        JvKind::False => "false".to_string(),
        JvKind::Number => jv_number_value(v).to_string(),
        JvKind::String => format!("\"{}\"", jv_string_value(v)),
        JvKind::Array => "[...]".to_string(),
        JvKind::Object => "{...}".to_string(),
        JvKind::Invalid => "invalid".to_string(),
    }
}
/// Convert timestamp to local time
pub fn f_localtime<T>(_jq: &mut JqState<T>, a: Jv) -> Jv {
    if jv_get_kind(&a) != JvKind::Number {
        return ret_error(a, jv_string("localtime() requires numeric inputs"));
    }
    let fsecs = jv_number_value(&a);
    let secs = fsecs as i64;
    jv_free(a);
    let tm = timestamp_to_tm(secs);
    let mut result = tm2jv(&tm);
    let frac = fsecs - fsecs.floor();
    let elem = jv_array_get(&result, 5);
    let current_secs = jv_number_value(&elem);
    jv_free(elem);
    result = jv_array_set(result, 5, jv_number(current_secs + frac));
    result
}
/// Convert Unix timestamp to Tm structure
fn timestamp_to_tm(secs: i64) -> Tm {
    let days = secs / 86400;
    let remaining = secs % 86400;
    let hours = (remaining / 3600) as i32;
    let minutes = ((remaining % 3600) / 60) as i32;
    let seconds = (remaining % 60) as i32;
    let mut year = 1970i32;
    let mut day_count = days;
    loop {
        let days_in_year = if is_leap_year(year as i64) { 366 } else { 365 };
        if day_count < days_in_year {
            break;
        }
        day_count -= days_in_year;
        year += 1;
    }
    let mut month = 0i32;
    let days_in_months = if is_leap_year(year as i64) {
        [31, 29, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31]
    } else {
        [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31]
    };
    for (i, &days_in_month) in days_in_months.iter().enumerate() {
        if day_count < days_in_month as i64 {
            month = i as i32;
            break;
        }
        day_count -= days_in_month as i64;
    }
    let mday = day_count as i32 + 1;
    let wday = ((days + 4) % 7) as i32;
    let mut yday = 0i64;
    for i in 0..month as usize {
        yday += days_in_months[i] as i64;
    }
    yday += day_count;
    Tm {
        tm_sec: seconds,
        tm_min: minutes,
        tm_hour: hours,
        tm_mday: mday,
        tm_mon: month,
        tm_year: year - 1900,
        tm_wday: wday,
        tm_yday: yday as i32,
        tm_isdst: 0,
        tm_gmtoff: 0,
        tm_zone: None,
    }
}
/// Format local time according to format string
pub fn f_strflocaltime<T>(_jq: &mut JqState<T>, a: Jv, b: Jv) -> Jv {
    let a = if jv_get_kind(&a) == JvKind::Number {
        f_localtime(_jq, a)
    } else if jv_get_kind(&a) != JvKind::Array {
        return ret_error2(
            a,
            b,
            jv_string("strflocaltime/1 requires parsed datetime inputs"),
        );
    } else {
        a
    };
    if jv_get_kind(&b) != JvKind::String {
        return ret_error2(a, b, jv_string("strflocaltime/1 requires a string format"));
    }
    let mut tm = Tm::default();
    if !jv2tm(&a, &mut tm) {
        return ret_error(
            b,
            jv_string("strflocaltime/1 requires parsed datetime inputs"),
        );
    }
    let fmt = jv_string_value(&b).to_string();
    jv_free(b);
    let result = strftime(&fmt, &tm);
    match result {
        Some(s) => jv_string(&s),
        None => jv_invalid_with_msg(jv_string("strflocaltime/1: unknown system failure")),
    }
}
/// Simple strftime implementation
fn strftime(fmt: &str, tm: &Tm) -> Option<String> {
    let mut result = String::new();
    let mut chars = fmt.chars().peekable();
    while let Some(c) = chars.next() {
        if c == '%' {
            if let Some(&spec) = chars.peek() {
                chars.next();
                match spec {
                    'Y' => result.push_str(&format!("{:04}", tm.tm_year + 1900)),
                    'm' => result.push_str(&format!("{:02}", tm.tm_mon + 1)),
                    'd' => result.push_str(&format!("{:02}", tm.tm_mday)),
                    'H' => result.push_str(&format!("{:02}", tm.tm_hour)),
                    'M' => result.push_str(&format!("{:02}", tm.tm_min)),
                    'S' => result.push_str(&format!("{:02}", tm.tm_sec)),
                    'j' => result.push_str(&format!("{:03}", tm.tm_yday + 1)),
                    'w' => result.push_str(&format!("{}", tm.tm_wday)),
                    '%' => result.push('%'),
                    _ => {
                        result.push('%');
                        result.push(spec);
                    }
                }
            } else {
                result.push('%');
            }
        } else {
            result.push(c);
        }
    }
    if result.is_empty() && !fmt.is_empty() {
        return None;
    }
    Some(result)
}
/// IEEE remainder (drem)
pub fn f_drem<T>(_jq: &mut JqState<T>, input: Jv, a: Jv, b: Jv) -> Jv {
    jv_free(input);
    if jv_get_kind(&a) != JvKind::Number {
        jv_free(b);
        return type_error(a, "number required");
    }
    if jv_get_kind(&b) != JvKind::Number {
        jv_free(a);
        return type_error(b, "number required");
    }
    let a_val = jv_number_value(&a);
    let b_val = jv_number_value(&b);
    let ret = jv_number(ieee_remainder(a_val, b_val));
    jv_free(a);
    jv_free(b);
    ret
}
/// IEEE remainder calculation
fn ieee_remainder(x: f64, y: f64) -> f64 {
    if y == 0.0 {
        return f64::NAN;
    }
    let quotient = (x / y).round();
    x - quotient * y
}
/// Inverse hyperbolic sine
pub fn f_asinh<T>(_jq: &mut JqState<T>, input: Jv) -> Jv {
    if jv_get_kind(&input) != JvKind::Number {
        return type_error(input, "number required");
    }
    let val = jv_number_value(&input);
    let ret = jv_number(val.asinh());
    jv_free(input);
    ret
}
/// Get keys from object or array
pub fn f_keys<T>(_jq: &mut JqState<T>, input: Jv) -> Jv {
    let kind = jv_get_kind(&input);
    if kind == JvKind::Object || kind == JvKind::Array {
        jv_keys(input)
    } else {
        type_error(input, "has no keys")
    }
}
/// Hypotenuse function
pub fn f_hypot<T>(_jq: &mut JqState<T>, input: Jv, a: Jv, b: Jv) -> Jv {
    jv_free(input);
    if jv_get_kind(&a) != JvKind::Number {
        jv_free(b);
        return type_error(a, "number required");
    }
    if jv_get_kind(&b) != JvKind::Number {
        jv_free(a);
        return type_error(b, "number required");
    }
    let a_val = jv_number_value(&a);
    let b_val = jv_number_value(&b);
    let ret = jv_number(a_val.hypot(b_val));
    jv_free(a);
    jv_free(b);
    ret
}
/// Floor function
pub fn f_floor<T>(_jq: &mut JqState<T>, input: Jv) -> Jv {
    if jv_get_kind(&input) != JvKind::Number {
        return type_error(input, "number required");
    }
    let val = jv_number_value(&input);
    let ret = jv_number(val.floor());
    jv_free(input);
    ret
}
/// Bessel function of second kind, order 1
pub fn f_y1<T>(_jq: &mut JqState<T>, input: Jv) -> Jv {
    if jv_get_kind(&input) != JvKind::Number {
        return type_error(input, "number required");
    }
    let val = jv_number_value(&input);
    let ret = jv_number(bessel_y1(val));
    jv_free(input);
    ret
}
/// Bessel function Y1 approximation
fn bessel_y1(x: f64) -> f64 {
    if x <= 0.0 {
        return f64::NEG_INFINITY;
    }
    if x < 8.0 {
        let y = x * x;
        let ans1 = x
            * (-0.4900604943e13
                + y
                    * (0.1275274390e13
                        + y
                            * (-0.5153438139e11
                                + y
                                    * (0.7349264551e9
                                        + y * (-0.4237922726e7 + y * 0.8511937935e4)))));
        let ans2 = 0.2499580570e14
            + y
                * (0.4244419664e12
                    + y
                        * (0.3733650367e10
                            + y
                                * (0.2245904002e8
                                    + y * (0.1020426050e6 + y * (0.3549632885e3 + y)))));
        (ans1 / ans2) + 0.636619772 * (bessel_j1(x) * x.ln() - 1.0 / x)
    } else {
        let z = 8.0 / x;
        let y = z * z;
        let xx = x - 2.356194491;
        let ans1 = 1.0
            + y
                * (0.183105e-2
                    + y
                        * (-0.3516396496e-4
                            + y * (0.2457520174e-5 + y * (-0.240337019e-6))));
        let ans2 = 0.04687499995
            + y
                * (-0.2002690873e-3
                    + y * (0.8449199096e-5 + y * (-0.88228987e-6 + y * 0.105787412e-6)));
        (0.636619772 / x).sqrt() * (ans1 * xx.sin() + z * ans2 * xx.cos())
    }
}
/// Compute the gamma function of the input
pub fn f_tgamma<T>(_jq: &mut JqState<T>, input: Jv) -> Jv {
    if jv_get_kind(&input) != JvKind::Number {
        return type_error(input, "number required");
    }
    let ret = jv_number(tgamma_impl(jv_number_value(&input)));
    jv_free(input);
    ret
}
/// Scale a floating-point number by a power of the radix
pub fn f_scalbln<T>(_jq: &mut JqState<T>, input: Jv, a: Jv, b: Jv) -> Jv {
    jv_free(input);
    if jv_get_kind(&a) != JvKind::Number {
        jv_free(b);
        return type_error(a, "number required");
    }
    if jv_get_kind(&b) != JvKind::Number {
        jv_free(a);
        return type_error(b, "number required");
    }
    let ret = jv_number(scalbln_impl(jv_number_value(&a), jv_number_value(&b) as i64));
    jv_free(a);
    jv_free(b);
    ret
}
/// Delete paths from a JSON value
pub fn f_delpaths<T>(_jq: &mut JqState<T>, a: Jv, b: Jv) -> Jv {
    jv_delpaths(a, b)
}
/// Bessel function of the first kind of integer order n
pub fn f_jn<T>(_jq: &mut JqState<T>, input: Jv, a: Jv, b: Jv) -> Jv {
    jv_free(input);
    if jv_get_kind(&a) != JvKind::Number {
        jv_free(b);
        return type_error(a, "number required");
    }
    if jv_get_kind(&b) != JvKind::Number {
        jv_free(a);
        return type_error(b, "number required");
    }
    let ret = jv_number(jn_impl(jv_number_value(&a) as i32, jv_number_value(&b)));
    jv_free(a);
    jv_free(b);
    ret
}
/// Binary equality operation
pub fn binop_equal(a: Jv, b: Jv) -> Jv {
    jv_bool(jv_equal(a, b))
}
/// Check equality of two values
pub fn f_equal<T>(_jq: &mut JqState<T>, input: Jv, a: Jv, b: Jv) -> Jv {
    jv_free(input);
    binop_equal(a, b)
}
/// Compute the natural logarithm of the input
pub fn f_log<T>(_jq: &mut JqState<T>, input: Jv) -> Jv {
    if jv_get_kind(&input) != JvKind::Number {
        return type_error(input, "number required");
    }
    let ret = jv_number(jv_number_value(&input).ln());
    jv_free(input);
    ret
}
/// Compute the next representable value toward a given direction
pub fn f_nexttoward<T>(_jq: &mut JqState<T>, input: Jv, a: Jv, b: Jv) -> Jv {
    jv_free(input);
    if jv_get_kind(&a) != JvKind::Number {
        jv_free(b);
        return type_error(a, "number required");
    }
    if jv_get_kind(&b) != JvKind::Number {
        jv_free(a);
        return type_error(b, "number required");
    }
    let x = jv_number_value(&a);
    let y = jv_number_value(&b);
    let ret = jv_number(nexttoward_impl(x, y));
    jv_free(a);
    jv_free(b);
    ret
}
/// Implementation of nexttoward (next representable value toward y)
fn nexttoward_impl(x: f64, y: f64) -> f64 {
    if x.is_nan() || y.is_nan() {
        return f64::NAN;
    }
    if x == y {
        return y;
    }
    if x == 0.0 {
        if y > 0.0 {
            return f64::MIN_POSITIVE * f64::EPSILON;
        } else {
            return -f64::MIN_POSITIVE * f64::EPSILON;
        }
    }
    let bits = x.to_bits();
    let next_bits = if (y > x) == (x > 0.0) { bits + 1 } else { bits - 1 };
    f64::from_bits(next_bits)
}
/// Compute the sine of the input
pub fn f_sin<T>(_jq: &mut JqState<T>, input: Jv) -> Jv {
    if jv_get_kind(&input) != JvKind::Number {
        return type_error(input, "number required");
    }
    let ret = jv_number(jv_number_value(&input).sin());
    jv_free(input);
    ret
}

// ============================================================================
// jq-defined function implementations (matching builtin.jq semantics)
// These are implemented in Rust for cases where the jq parser doesn't work
// ============================================================================

/// add: reduce .[] as $x (null; . + $x)
/// Sums array elements using the + operator semantics
pub fn f_add<T>(_jq: &mut JqState<T>, input: Jv) -> Jv {
    if input.get_kind() != JvKind::Array {
        return type_error(input, "cannot add non-array");
    }
    let len = jv_array_length(&input);
    if len == 0 {
        jv_free(input);
        return Jv::null();
    }

    // Start with null, then reduce with +
    let mut acc = Jv::null();
    for i in 0..len {
        let elem = crate::jv::jv_array_get(jv_copy(&input), i);
        acc = binop_plus(acc, elem);
        if !acc.is_valid() {
            jv_free(input);
            return acc;
        }
    }
    jv_free(input);
    acc
}

/// unique: group_by(.) | map(.[0])
/// Returns sorted array with duplicates removed
pub fn f_unique<T>(_jq: &mut JqState<T>, input: Jv) -> Jv {
    if input.get_kind() != JvKind::Array {
        return type_error(input, "cannot unique non-array");
    }

    // Sort first
    let sorted = jv_sort(jv_copy(&input), Jv::null());
    if !sorted.is_valid() {
        jv_free(input);
        return sorted;
    }

    // Remove consecutive duplicates
    let len = jv_array_length(&sorted);
    if len == 0 {
        jv_free(input);
        return sorted;
    }

    let mut result = crate::jv::jv_array();
    let mut prev = crate::jv::jv_array_get(jv_copy(&sorted), 0);
    result = crate::jv::jv_array_append(result, jv_copy(&prev));

    for i in 1..len {
        let curr = crate::jv::jv_array_get(jv_copy(&sorted), i);
        if !jv_equal(jv_copy(&prev), jv_copy(&curr)) {
            result = crate::jv::jv_array_append(result, jv_copy(&curr));
            jv_free(prev);
            prev = curr;
        } else {
            jv_free(curr);
        }
    }
    jv_free(prev);
    jv_free(sorted);
    jv_free(input);
    result
}

/// Helper for flatten
fn flatten_impl(input: Jv, depth: i32) -> Jv {
    if input.get_kind() != JvKind::Array {
        return crate::jv::jv_array_append(crate::jv::jv_array(), input);
    }

    let len = jv_array_length(&input);
    let mut result = crate::jv::jv_array();

    for i in 0..len {
        let elem = crate::jv::jv_array_get(jv_copy(&input), i);
        if elem.get_kind() == JvKind::Array && depth != 0 {
            let flattened = flatten_impl(elem, if depth > 0 { depth - 1 } else { depth });
            let flen = jv_array_length(&flattened);
            for j in 0..flen {
                let e = crate::jv::jv_array_get(jv_copy(&flattened), j);
                result = crate::jv::jv_array_append(result, e);
            }
            jv_free(flattened);
        } else {
            result = crate::jv::jv_array_append(result, elem);
        }
    }
    jv_free(input);
    result
}

/// flatten($depth): Flatten nested arrays to specified depth (-1 = infinite)
pub fn f_flatten<T>(_jq: &mut JqState<T>, input: Jv, depth: Jv) -> Jv {
    if input.get_kind() != JvKind::Array {
        jv_free(depth);
        return type_error(input, "cannot flatten non-array");
    }

    let d = if depth.get_kind() == JvKind::Number {
        depth.number_value() as i32
    } else {
        jv_free(depth);
        -1 // infinite depth
    };

    if d < -1 {
        jv_free(input);
        return Jv::invalid_with_msg(Jv::string("flatten depth must not be negative"));
    }

    flatten_impl(input, d)
}

/// ltrimstr($left): if startswith($left) then .[$left | length:] end
pub fn f_ltrimstr<T>(_jq: &mut JqState<T>, input: Jv, left: Jv) -> Jv {
    if input.get_kind() != JvKind::String {
        jv_free(left);
        return type_error(input, "ltrimstr requires string input");
    }
    if left.get_kind() != JvKind::String {
        jv_free(input);
        return type_error(left, "ltrimstr requires string argument");
    }

    let input_str = input.string_value().unwrap_or("");
    let left_str = left.string_value().unwrap_or("");

    if input_str.starts_with(left_str) {
        let result = Jv::string(&input_str[left_str.len()..]);
        jv_free(input);
        jv_free(left);
        result
    } else {
        jv_free(left);
        input // return unchanged
    }
}

/// rtrimstr($right): if endswith($right) then .[:$right | -length] end
pub fn f_rtrimstr<T>(_jq: &mut JqState<T>, input: Jv, right: Jv) -> Jv {
    if input.get_kind() != JvKind::String {
        jv_free(right);
        return type_error(input, "rtrimstr requires string input");
    }
    if right.get_kind() != JvKind::String {
        jv_free(input);
        return type_error(right, "rtrimstr requires string argument");
    }

    let input_str = input.string_value().unwrap_or("");
    let right_str = right.string_value().unwrap_or("");

    if input_str.ends_with(right_str) {
        let new_len = input_str.len() - right_str.len();
        let result = Jv::string(&input_str[..new_len]);
        jv_free(input);
        jv_free(right);
        result
    } else {
        jv_free(right);
        input // return unchanged
    }
}

/// ascii_downcase: explode | map(if 65 <= . and . <= 90 then . + 32 else . end) | implode
pub fn f_ascii_downcase<T>(_jq: &mut JqState<T>, input: Jv) -> Jv {
    if input.get_kind() != JvKind::String {
        return type_error(input, "ascii_downcase requires string");
    }

    let s = input.string_value().unwrap_or("");
    let result: String = s.chars().map(|c| {
        if c >= 'A' && c <= 'Z' {
            ((c as u8) + 32) as char
        } else {
            c
        }
    }).collect();

    jv_free(input);
    Jv::string(&result)
}

/// ascii_upcase: explode | map(if 97 <= . and . <= 122 then . - 32 else . end) | implode
pub fn f_ascii_upcase<T>(_jq: &mut JqState<T>, input: Jv) -> Jv {
    if input.get_kind() != JvKind::String {
        return type_error(input, "ascii_upcase requires string");
    }

    let s = input.string_value().unwrap_or("");
    let result: String = s.chars().map(|c| {
        if c >= 'a' && c <= 'z' {
            ((c as u8) - 32) as char
        } else {
            c
        }
    }).collect();

    jv_free(input);
    Jv::string(&result)
}

// Block::default() is provided by compile.rs via #[derive(Default)]
