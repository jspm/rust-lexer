use std::convert::TryFrom;

pub enum Import {
  Dynamic(DynamicImport),
  Static(StaticImport),
}

pub struct DynamicImport {
  pub statement_start: usize,
  pub start: usize,
  pub end: usize,
  pub dynamic: isize,
}

pub struct StaticImport {
  pub statement_start: usize,
  pub start: usize,
  pub end: usize,
  pub statement_end: usize,
}

pub struct Export {
  pub statement_start: usize,
  pub start: usize,
  pub end: usize,
}

impl Export {
  pub fn to_string<'a> (self: &Export, source: &'a str) -> &'a str {
    return &source[self.start..self.end];
  }
}

pub struct SourceAnalysis {
  pub imports: Vec<Import>,
  pub exports: Vec<Export>,
}

#[derive(std::fmt::Debug)]
pub struct ParseError {
  pub idx: usize,
  pub line: usize,
  pub col: usize,
}

impl ParseError {
  pub fn fromIndex (source: &'a str, idx: usize) -> ParseError {
    let line: usize = 0; // source.slice(0, wasm.e()).split('\n').length
    let col: usize = 0; // wasm.e() - source.lastIndexOf('\n', wasm.e() - 1
    return ParseError { idx, line, col };
  }
}
ParseError::fromIndex(3)

pub fn parse (input: &str) -> Result<SourceAnalysis, ParseError> {
  let mut analysis = SourceAnalysis {
    imports: Vec::with_capacity(20),
    exports: Vec::with_capacity(20),
  };

  let mut templateStack = Vec::<usize>::with_capacity(5);
  let mut openTokenIndexStack = Vec::<usize>::with_capacity(50);

  let mut templateStackDepth: usize = 0;
  let mut openTokenDepth: usize = 0;
  let mut templateDepth: isize = -1;
  let mut lastTokenIndex: usize;

  let source = input.as_bytes();

  let mut first = false;
  let mut i: usize = 0;
  let len = source.len();
  while i < len - 1 {
    if first {
      first = false;
    }
    else {
      i += 1;
    }
    let ch = source[i];

    if ch == ' ' as u8 || ch < 14 && ch > 8 {
      continue;
    }

    match ch as char {
      'e' => {

      },
      'i' => {

      },
      '(' => {

      },
      ')' => {

      },
      '{' => {

      },
      '}' => {

      },
      '\'' => {

      },
      '"' => {

      },
      '/' => {

      },
      '`' => {

      },
      _ => {}
    }
    lastTokenIndex = i;
  }

  if templateDepth != -1 || openTokenDepth {
    return Err(ParseError::fromIndex(source, idx));
  }

  // analysis.exports.push(Export { statement_start: 0, start: 0, end: 5, dynamic: -1 }
  // analysis.imports.push(Import { start: 6, end: 11 }
  Ok(analysis)
}

pub fn main () {
  let source = "hello world";

  let analysis = parse(source).expect("Parse error");

  for import in analysis.imports {
    let start = match &import {
      Import::Dynamic(impt) => impt.start,
      Import::Static(impt) => impt.start,
    };
    let end = match &import {
      Import::Dynamic(impt) => impt.end,
      Import::Static(impt) => impt.end,
    };
    println!("Import: {}", std::str::from_utf8(&source.as_bytes()[start..end]).expect("Invalid utf8"));
  }

  for export in analysis.exports {
    let start = export.start;
    let end = export.end;
    println!("Export: {}", std::str::from_utf8(&source.as_bytes()[start..end]).expect("Invalid utf8"));
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn invalid_string () {
    let source = r#"import './export.js';

import d from './export.js';

import { s as p } from './reexport1.js';

import { z, q as r } from './reexport2.js';

   '

import * as q from './reexport1.js';

export { d as a, p as b, z as c, r as d, q }"#;
    
    let err = parse(source).err().unwrap();
    assert_eq!(err.line, 9);
    assert_eq!(err.col, 5);
  }

  #[test]
  fn invalid_export () {
    let source = r#"export { a = };"#;
    let err = parse(source).err().expect("Should error");
    assert_eq!(err.idx, 11);
  }

  #[test]
  fn single_parse_cases () {
    parse("export { x }").unwrap();
    parse("'asdf'").unwrap();
    parse("/asdf/").unwrap();
    parse("`asdf`").unwrap();
    parse("/**/").unwrap();
    parse("//").unwrap();
  }

  #[test]
  fn simple_export_with_unicode_conversions () {
    let source = r#"export var p𓀀s,q"#;
    let SourceAnalysis { imports, exports, .. } = parse(source).unwrap();
    assert_eq!(imports.len(), 0);
    assert_eq!(exports.len(), 2);
    assert_eq!(exports[0].to_string(source), "p𓀀s");
    assert_eq!(exports[1].to_string(source), "q");
  }

//   #[test]
//   fn simple_import () {
//     let source = r#"
//       import test from "test";
//       console.log(test);
//     "#;
//     let SourceAnalysis { imports, exports } = parse(source).unwrap();
//     assert_eq!(imports.len(), 1);
//     const { s, e, ss, se, d } = imports[0];
//     assert_eq!(d, -1);
//     assert_eq!(source.slice(s, e), "test");
//     assert_eq!(source.slice(ss, se), "import test from "test"");
//     assert_eq!(exports.len(), 0);
//   }

//   #[test]
//   fn import_export_with_comments () {
//     let source = r#"

//       import/* 'x' */ 'a';

//       import /* 'x' */ 'b';

//       export var z  /*  */  
//       export {
//         a,
//         // b,
//         /* c */ d
//       };
//     "#;
//     let SourceAnalysis { imports, exports } = parse(source).unwrap();
//     assert_eq!(imports.len(), 2);
//     assert_eq!(source[imports[0].start..imports[0].end], "a");
//     assert_eq!(source.slice(imports[0].statement_start, imports[0].statement_end), `import/* 'x' */ 'a'`);
//     assert_eq!(source.slice(imports[1].s, imports[1].e), "b");
//     assert_eq!(source.slice(imports[1].statement_start, imports[1].statement_end), `import /* 'x' */ 'b'`);
//     assert_eq!(exports.toString(), "z,a,d");
//   }

//   #[test]
//   fn exported_function () {
//     let source = r#"
//       export function a𓀀 () {

//       }
//       export class Q{

//       }
//     "#;
//     const [, exports] = parse(source);
//     assert_eq!(exports[0], "a𓀀");
//     assert_eq!(exports[1], "Q");
//   }

//   #[test]
//   fn export_destructuring () {
//     let source = r#"
//       export const { a, b } = foo;

//       export { ok };
//     "#;
//     const [, exports] = parse(source);
//     assert_eq!(exports[0], "ok");
//   }

//   #[test]
//   fn minified_import_syntax () {
//     let source = r#"import{TemplateResult as t}from"lit-html";import{a as e}from"./chunk-4be41b30.js";export{j as SVGTemplateResult,i as TemplateResult,g as html,h as svg}from"./chunk-4be41b30.js";window.JSCompiler_renameProperty='asdf';"#;
//     let SourceAnalysis { imports, exports } = parse(source).unwrap();
//     assert_eq!(imports.len(), 3);
//     assert_eq!(imports[0].s, 32);
//     assert_eq!(imports[0].e, 40);
//     assert_eq!(imports[0].statement_start, 0);
//     assert_eq!(imports[0].statement_end, 41);
//     assert_eq!(imports[1].s, 61);
//     assert_eq!(imports[1].e, 80);
//     assert_eq!(imports[1].statement_start, 42);
//     assert_eq!(imports[1].statement_end, 81);
//     assert_eq!(imports[2].s, 156);
//     assert_eq!(imports[2].e, 175);
//     assert_eq!(imports[2].statement_start, 82);
//     assert_eq!(imports[2].statement_end, 176);
//   }

//   #[test]
//   fn more_minified_imports () {
//     let source = r#"import"some/import.js";`
//     let SourceAnalysis { imports, exports } = parse(source).unwrap();
//     assert_eq!(imports.len(), 1);
//     assert_eq!(imports[0].s, 7);
//     assert_eq!(imports[0].e, 21);
//     assert_eq!(imports[0].statement_start, 0);
//     assert_eq!(imports[0].statement_end, 22);
//   }

//   #[test]
//   fn return_bracket_division () {
//     let source = r#"function variance(){return s/(a-1)}"#;
//     let SourceAnalysis { imports, exports } = parse(source).unwrap();
//   }

//   #[test]
//   fn simple_reexport () {
//     let source = r#"
//       export { hello as default } from "test-dep";
//     "#;
//     let SourceAnalysis { imports, exports } = parse(source).unwrap();
//     assert_eq!(imports.len(), 1);
//     const { s, e, ss, se, d } = imports[0];
//     assert_eq!(d, -1);
//     assert_eq!(source.slice(s, e), "test-dep");
//     assert_eq!(source.slice(ss, se), "export { hello as default } from "test-dep"");

//     assert_eq!(exports.len(), 1);
//     assert_eq!(exports[0], "default");
//   }

//   #[test]
//   fn import_meta () {
//     let source = r#"
//       export var hello = 'world';
//       console.log(import.meta.url);
//     "#;
//     let SourceAnalysis { imports, exports } = parse(source).unwrap();
//     assert_eq!(imports.len(), 1);
//     const { s, e, ss, se, d } = imports[0];
//     assert_eq!(d, -2);
//     assert_eq!(ss, 53);
//     assert_eq!(se, 64);
//     assert_eq!(source.slice(s, e), "import.meta");
//   }

//   #[test]
//   fn import_meta_edge_cases () {
//     let source = r#"
//       // Import meta
//       import.
//        meta
//       // Not import meta
//       a.
//       import.
//         meta
//     "#;
//     let SourceAnalysis { imports, exports } = parse(source).unwrap();
//     assert_eq!(imports.len(), 1);
//     const { s, e, ss, se, d } = imports[0];
//     assert_eq!(d, -2);
//     assert_eq!(ss, 28);
//     assert_eq!(se, 47);
//     assert_eq!(source.slice(s, e), "import.\n       meta");
//   }

//   #[test]
//   fn dynamic_import_method () {
//     await init;
//     let source = r#"
//       class A {
//         import() {
//         }
//       }
//     "#;
//     const [imports] = parse(source);
//     assert_eq!(imports.len(), 0);
//   }

//   #[test]
//   fn dynamic_import_edge_cases () {
//     let source = r#"
//       ({
//         // not a dynamic import!
//         import(not1) {}
//       }
//       {
//         // is a dynamic import!
//         import(is1);
//       }
//       a.
//       // not a dynamic import!
//       import(not2);
//       a.
//       b()
//       // is a dynamic import!
//       import(is2);

//       const myObject = {
//         import: ()=> import(some_url)
//       }
//     "#;
//     let SourceAnalysis { imports, exports } = parse(source).unwrap();
//     assert_eq!(imports.len(), 3);
//     var { s, e, ss, se, d } = imports[0];
//     assert_eq!(ss, d);
//     assert_eq!(se, 0);
//     assert_eq!(source.substr(d, 6), "import");
//     assert_eq!(source.slice(s, e), "is1");

//     var { s, e, ss, se, d } = imports[1];
//     assert_eq!(ss, d);
//     assert_eq!(se, 0);
//     assert_eq!(source.slice(s, e), "is2");

//     var { s, e, ss, se, d } = imports[2];
//     assert_eq!(ss, d);
//     assert_eq!(se, 0);
//     assert_eq!(source.slice(s, e), "some_url");
//   }

//   #[test]
//   fn import_after_code () {
//     let source = r#"
//       export function f () {
//         g();
//       }

//       import { g } from './test-circular2.js';
//     "#;
//     let SourceAnalysis { imports, exports } = parse(source).unwrap();
//     assert_eq!(imports.len(), 1);
//     const { s, e, ss, se, d } = imports[0];
//     assert_eq!(d, -1);
//     assert_eq!(source.slice(s, e), "./test-circular2.js");
//     assert_eq!(source.slice(ss, se), `import { g } from './test-circular2.js'`);
//     assert_eq!(exports.len(), 1);
//     assert_eq!(exports[0], "f");
//   }

//   #[test]
//   fn comments () {
//     let source = r#"/*
//     VERSION
//   */import util from 'util';

// //
// function x() {
// }

//       /**/
//       // '
//       /* / */
//       /*

//          * export { b }
//       \\*/export { a }

//       function () {
//         /***/
//       }
//     `
//     let SourceAnalysis { imports, exports } = parse(source).unwrap();
//     assert_eq!(imports.len(), 1);
//     assert_eq!(source.slice(imports[0].s, imports[0].e), "util");
//     assert_eq!(source.slice(imports[0].statement_start, imports[0].statement_end), `import util from 'util'`);
//     assert_eq!(exports.len(), 1);
//     assert_eq!(exports[0], "a");
//   }

//   #[test]
//   fn strings () {
//     let source = r#"
//       "";
//       \`
//         \${
//           import(\`test/\${ import(b)}\`); /*
//               \`  }
//           */
//         }
//       \`
//       export { a }
//     "#;
//     let SourceAnalysis { imports, exports } = parse(source).unwrap();
//     assert_eq!(imports.len(), 2);
//     assert.notEqual(imports[0].d, -1);
//     assert_eq!(imports[0].statement_start, imports[0].d);
//     assert_eq!(imports[0].statement_end, 0);
//     assert_eq!(source.slice(imports[0].d, imports[0].s), "import(");
//     assert.notEqual(imports[1].d, -1);
//     assert_eq!(imports[1].statement_start, imports[1].d);
//     assert_eq!(imports[1].statement_end, 0);
//     assert_eq!(source.slice(imports[1].d, imports[1].s), "import(");
//     assert_eq!(exports.len(), 1);
//     assert_eq!(exports[0], "a");
//   }

//   #[test]
//   fn bracket_matching () {
//     pars")"
//       instance.extend('parseExprAtom', function (nextMethod) {
//         return function () {
//           function parseExprAtom(refDestructuringErrors) {
//             if (this.type === tt._import) {
//               return parseDynamicImport.call(this);
//             }
//             return c(refDestructuringErrors);
//           }
//         }();
//       }
//       export { a }
//     `);
//   }

//   #[test]
//   fn division_regex_ambiguity () {
//     let source = r#"
//       /as)df/; x();
//       a / 2; '  /  '
//       while (true)
//         /test'/
//       x-/a'/g
//       finally{}/a'/g
//       (){}/d'export { b }/g
//       ;{}/e'/g;
//       {}/f'/g
//       a / 'b' / c;
//       /a'/ - /b'/;
//       +{} /g -'/g'
//       ('a')/h -'/g'
//       if //x
//       ('a')/i'/g;
//       /asdf/ / /as'df/; // '
//       \`\${/test/ + 5}\`
//       /regex/ / x;
//       function () {
//         return /*asdf8*// 5/;
//       }
//       export { a };
//     "#;
//     let SourceAnalysis { imports, exports } = parse(source).unwrap();
//     assert_eq!(imports.len(), 0);
//     assert_eq!(exports.len(), 1);
//     assert_eq!(exports[0], "a");
//   }

//   #[test]
//   fn template_string_expression_ambiguity() {
//     let source = r#"
//       \`$\`
//       import 'a';
//       \`\`
//       export { b };
//       \`a$b\`
//       import(\`$\`);
//       \`{$}\`
//     "#;
//     let analysis = parse(source).unwrap();
//     assert_eq!(analysis.imports.len(), 2);
//     assert_eq!(exports.len(), 1);
//     assert_eq!(exports[0], "b");
//   }

//   #[test]
//   fn many_exports() {
//     let source = r#"
//       export { _iconsCache as fas, prefix, faAbacus, faAcorn, faAd, faAddressBook, faAddressCard, faAdjust, faAirFreshener, faAlarmClock, faAlarmExclamation, faAlarmPlus, faAlarmSnooze, faAlicorn, faAlignCenter, faAlignJustify, faAlignLeft, faAlignRight, faAlignSlash, faAllergies, faAmbulance, faAmericanSignLanguageInterpreting, faAnalytics, faAnchor, faAngel, faAngleDoubleDown, faAngleDoubleLeft, faAngleDoubleRight, faAngleDoubleUp, faAngleDown, faAngleLeft, faAngleRight, faAngleUp, faAngry, faAnkh, faAppleAlt, faAppleCrate, faArchive, faArchway, faArrowAltCircleDown, faArrowAltCircleLeft, faArrowAltCircleRight, faArrowAltCircleUp, faArrowAltDown, faArrowAltFromBottom, faArrowAltFromLeft, faArrowAltFromRight, faArrowAltFromTop, faArrowAltLeft, faArrowAltRight, faArrowAltSquareDown, faArrowAltSquareLeft, faArrowAltSquareRight, faArrowAltSquareUp, faArrowAltToBottom, faArrowAltToLeft, faArrowAltToRight, faArrowAltToTop, faArrowAltUp, faArrowCircleDown, faArrowCircleLeft, faArrowCircleRight, faArrowCircleUp, faArrowDown, faArrowFromBottom, faArrowFromLeft, faArrowFromRight, faArrowFromTop, faArrowLeft, faArrowRight, faArrowSquareDown, faArrowSquareLeft, faArrowSquareRight, faArrowSquareUp, faArrowToBottom, faArrowToLeft, faArrowToRight, faArrowToTop, faArrowUp, faArrows, faArrowsAlt, faArrowsAltH, faArrowsAltV, faArrowsH, faArrowsV, faAssistiveListeningSystems, faAsterisk, faAt, faAtlas, faAtom, faAtomAlt, faAudioDescription, faAward, faAxe, faAxeBattle, faBaby, faBabyCarriage, faBackpack, faBackspace, faBackward, faBacon, faBadge, faBadgeCheck, faBadgeDollar, faBadgePercent, faBadgerHoney, faBagsShopping, faBalanceScale, faBalanceScaleLeft, faBalanceScaleRight, faBallPile, faBallot, faBallotCheck, faBan, faBandAid, faBarcode, faBarcodeAlt, faBarcodeRead, faBarcodeScan, faBars, faBaseball, faBaseballBall, faBasketballBall, faBasketballHoop, faBat, faBath, faBatteryBolt, faBatteryEmpty, faBatteryFull, faBatteryHalf, faBatteryQuarter, faBatterySlash, faBatteryThreeQuarters, faBed, faBeer, faBell, faBellExclamation, faBellPlus, faBellSchool, faBellSchoolSlash, faBellSlash, faBells, faBezierCurve, faBible, faBicycle, faBiking, faBikingMountain, faBinoculars, faBiohazard, faBirthdayCake, faBlanket, faBlender, faBlenderPhone, faBlind, faBlog, faBold, faBolt, faBomb, faBone, faBoneBreak, faBong, faBook, faBookAlt, faBookDead, faBookHeart, faBookMedical, faBookOpen, faBookReader, faBookSpells, faBookUser, faBookmark, faBooks, faBooksMedical, faBoot, faBoothCurtain, faBorderAll, faBorderBottom, faBorderCenterH, faBorderCenterV, faBorderInner, faBorderLeft, faBorderNone, faBorderOuter, faBorderRight, faBorderStyle, faBorderStyleAlt, faBorderTop, faBowArrow, faBowlingBall, faBowlingPins, faBox, faBoxAlt, faBoxBallot, faBoxCheck, faBoxFragile, faBoxFull, faBoxHeart, faBoxOpen, faBoxUp, faBoxUsd, faBoxes, faBoxesAlt, faBoxingGlove, faBrackets, faBracketsCurly, faBraille, faBrain, faBreadLoaf, faBreadSlice, faBriefcase, faBriefcaseMedical, faBringForward, faBringFront, faBroadcastTower, faBroom, faBrowser, faBrush, faBug, faBuilding, faBullhorn, faBullseye, faBullseyeArrow, faBullseyePointer, faBurgerSoda, faBurn, faBurrito, faBus, faBusAlt, faBusSchool, faBusinessTime, faCabinetFiling, faCalculator, faCalculatorAlt, faCalendar, faCalendarAlt, faCalendarCheck, faCalendarDay, faCalendarEdit, faCalendarExclamation, faCalendarMinus, faCalendarPlus, faCalendarStar, faCalendarTimes, faCalendarWeek, faCamera, faCameraAlt, faCameraRetro, faCampfire, faCampground, faCandleHolder, faCandyCane, faCandyCorn, faCannabis, faCapsules, faCar, faCarAlt, faCarBattery, faCarBuilding, faCarBump, faCarBus, faCarCrash, faCarGarage, faCarMechanic, faCarSide, faCarTilt, faCarWash, faCaretCircleDown, faCaretCircleLeft, faCaretCircleRight, faCaretCircleUp, faCaretDown, faCaretLeft, faCaretRight, faCaretSquareDown, faCaretSquareLeft, faCaretSquareRight, faCaretSquareUp, faCaretUp, faCarrot, faCars, faCartArrowDown, faCartPlus, faCashRegister, faCat, faCauldron, faCertificate, faChair, faChairOffice, faChalkboard, faChalkboardTeacher, faChargingStation, faChartArea, faChartBar, faChartLine, faChartLineDown, faChartNetwork, faChartPie, faChartPieAlt, faChartScatter, faCheck, faCheckCircle, faCheckDouble, faCheckSquare, faCheese, faCheeseSwiss, faCheeseburger, faChess, faChessBishop, faChessBishopAlt, faChessBoard, faChessClock, faChessClockAlt, faChessKing, faChessKingAlt, faChessKnight, faChessKnightAlt, faChessPawn, faChessPawnAlt, faChessQueen, faChessQueenAlt, faChessRook, faChessRookAlt, faChevronCircleDown, faChevronCircleLeft, faChevronCircleRight, faChevronCircleUp, faChevronDoubleDown, faChevronDoubleLeft, faChevronDoubleRight, faChevronDoubleUp, faChevronDown, faChevronLeft, faChevronRight, faChevronSquareDown, faChevronSquareLeft, faChevronSquareRight, faChevronSquareUp, faChevronUp, faChild, faChimney, faChurch, faCircle, faCircleNotch, faCity, faClawMarks, faClinicMedical, faClipboard, faClipboardCheck, faClipboardList, faClipboardListCheck, faClipboardPrescription, faClipboardUser, faClock, faClone, faClosedCaptioning, faCloud, faCloudDownload, faCloudDownloadAlt, faCloudDrizzle, faCloudHail, faCloudHailMixed, faCloudMeatball, faCloudMoon, faCloudMoonRain, faCloudRain, faCloudRainbow, faCloudShowers, faCloudShowersHeavy, faCloudSleet, faCloudSnow, faCloudSun, faCloudSunRain, faCloudUpload, faCloudUploadAlt, faClouds, faCloudsMoon, faCloudsSun, faClub, faCocktail, faCode, faCodeBranch, faCodeCommit, faCodeMerge, faCoffee, faCoffeeTogo, faCoffin, faCog, faCogs, faCoin, faCoins, faColumns, faComment, faCommentAlt, faCommentAltCheck, faCommentAltDollar, faCommentAltDots, faCommentAltEdit, faCommentAltExclamation, faCommentAltLines, faCommentAltMedical, faCommentAltMinus, faCommentAltPlus, faCommentAltSlash, faCommentAltSmile, faCommentAltTimes, faCommentCheck, faCommentDollar, faCommentDots, faCommentEdit, faCommentExclamation, faCommentLines, faCommentMedical, faCommentMinus, faCommentPlus, faCommentSlash, faCommentSmile, faCommentTimes, faComments, faCommentsAlt, faCommentsAltDollar, faCommentsDollar, faCompactDisc, faCompass, faCompassSlash, faCompress, faCompressAlt, faCompressArrowsAlt, faCompressWide, faConciergeBell, faConstruction, faContainerStorage, faConveyorBelt, faConveyorBeltAlt, faCookie, faCookieBite, faCopy, faCopyright, faCorn, faCouch, faCow, faCreditCard, faCreditCardBlank, faCreditCardFront, faCricket, faCroissant, faCrop, faCropAlt, faCross, faCrosshairs, faCrow, faCrown, faCrutch, faCrutches, faCube, faCubes, faCurling, faCut, faDagger, faDatabase, faDeaf, faDebug, faDeer, faDeerRudolph, faDemocrat, faDesktop, faDesktopAlt, faDewpoint, faDharmachakra, faDiagnoses, faDiamond, faDice, faDiceD10, faDiceD12, faDiceD20, faDiceD4, faDiceD6, faDiceD8, faDiceFive, faDiceFour, faDiceOne, faDiceSix, faDiceThree, faDiceTwo, faDigging, faDigitalTachograph, faDiploma, faDirections, faDisease, faDivide, faDizzy, faDna, faDoNotEnter, faDog, faDogLeashed, faDollarSign, faDolly, faDollyEmpty, faDollyFlatbed, faDollyFlatbedAlt, faDollyFlatbedEmpty, faDonate, faDoorClosed, faDoorOpen, faDotCircle, faDove, faDownload, faDraftingCompass, faDragon, faDrawCircle, faDrawPolygon, faDrawSquare, faDreidel, faDrone, faDroneAlt, faDrum, faDrumSteelpan, faDrumstick, faDrumstickBite, faDryer, faDryerAlt, faDuck, faDumbbell, faDumpster, faDumpsterFire, faDungeon, faEar, faEarMuffs, faEclipse, faEclipseAlt, faEdit, faEgg, faEggFried, faEject, faElephant, faEllipsisH, faEllipsisHAlt, faEllipsisV, faEllipsisVAlt, faEmptySet, faEngineWarning, faEnvelope, faEnvelopeOpen, faEnvelopeOpenDollar, faEnvelopeOpenText, faEnvelopeSquare, faEquals, faEraser, faEthernet, faEuroSign, faExchange, faExchangeAlt, faExclamation, faExclamationCircle, faExclamationSquare, faExclamationTriangle, faExpand, faExpandAlt, faExpandArrows, faExpandArrowsAlt, faExpandWide, faExternalLink, faExternalLinkAlt, faExternalLinkSquare, faExternalLinkSquareAlt, faEye, faEyeDropper, faEyeEvil, faEyeSlash, faFan, faFarm, faFastBackward, faFastForward, faFax, faFeather, faFeatherAlt, faFemale, faFieldHockey, faFighterJet, faFile, faFileAlt, faFileArchive, faFileAudio, faFileCertificate, faFileChartLine, faFileChartPie, faFileCheck, faFileCode, faFileContract, faFileCsv, faFileDownload, faFileEdit, faFileExcel, faFileExclamation, faFileExport, faFileImage, faFileImport, faFileInvoice, faFileInvoiceDollar, faFileMedical, faFileMedicalAlt, faFileMinus, faFilePdf, faFilePlus, faFilePowerpoint, faFilePrescription, faFileSearch, faFileSignature, faFileSpreadsheet, faFileTimes, faFileUpload, faFileUser, faFileVideo, faFileWord, faFilesMedical, faFill, faFillDrip, faFilm, faFilmAlt, faFilter, faFingerprint, faFire, faFireAlt, faFireExtinguisher, faFireSmoke, faFireplace, faFirstAid, faFish, faFishCooked, faFistRaised, faFlag, faFlagAlt, faFlagCheckered, faFlagUsa, faFlame, faFlask, faFlaskPoison, faFlaskPotion, faFlower, faFlowerDaffodil, faFlowerTulip, faFlushed, faFog, faFolder, faFolderMinus, faFolderOpen, faFolderPlus, faFolderTimes, faFolderTree, faFolders, faFont, faFontAwesomeLogoFull, faFontCase, faFootballBall, faFootballHelmet, faForklift, faForward, faFragile, faFrenchFries, faFrog, faFrostyHead, faFrown, faFrownOpen, faFunction, faFunnelDollar, faFutbol, faGameBoard, faGameBoardAlt, faGamepad, faGasPump, faGasPumpSlash, faGavel, faGem, faGenderless, faGhost, faGift, faGiftCard, faGifts, faGingerbreadMan, faGlass, faGlassChampagne, faGlassCheers, faGlassCitrus, faGlassMartini, faGlassMartiniAlt, faGlassWhiskey, faGlassWhiskeyRocks, faGlasses, faGlassesAlt, faGlobe, faGlobeAfrica, faGlobeAmericas, faGlobeAsia, faGlobeEurope, faGlobeSnow, faGlobeStand, faGolfBall, faGolfClub, faGopuram, faGraduationCap, faGreaterThan, faGreaterThanEqual, faGrimace, faGrin, faGrinAlt, faGrinBeam, faGrinBeamSweat, faGrinHearts, faGrinSquint, faGrinSquintTears, faGrinStars, faGrinTears, faGrinTongue, faGrinTongueSquint, faGrinTongueWink, faGrinWink, faGripHorizontal, faGripLines, faGripLinesVertical, faGripVertical, faGuitar, faHSquare, faH1, faH2, faH3, faH4, faHamburger, faHammer, faHammerWar, faHamsa, faHandHeart, faHandHolding, faHandHoldingBox, faHandHoldingHeart, faHandHoldingMagic, faHandHoldingSeedling, faHandHoldingUsd, faHandHoldingWater, faHandLizard, faHandMiddleFinger, faHandPaper, faHandPeace, faHandPointDown, faHandPointLeft, faHandPointRight, faHandPointUp, faHandPointer, faHandReceiving, faHandRock, faHandScissors, faHandSpock, faHands, faHandsHeart, faHandsHelping, faHandsUsd, faHandshake, faHandshakeAlt, faHanukiah, faHardHat, faHashtag, faHatChef, faHatSanta, faHatWinter, faHatWitch, faHatWizard, faHaykal, faHdd, faHeadSide, faHeadSideBrain, faHeadSideMedical, faHeadVr, faHeading, faHeadphones, faHeadphonesAlt, faHeadset, faHeart, faHeartBroken, faHeartCircle, faHeartRate, faHeartSquare, faHeartbeat, faHelicopter, faHelmetBattle, faHexagon, faHighlighter, faHiking, faHippo, faHistory, faHockeyMask, faHockeyPuck, faHockeySticks, faHollyBerry, faHome, faHomeAlt, faHomeHeart, faHomeLg, faHomeLgAlt, faHoodCloak, faHorizontalRule, faHorse, faHorseHead, faHospital, faHospitalAlt, faHospitalSymbol, faHospitalUser, faHospitals, faHotTub, faHotdog, faHotel, faHourglass, faHourglassEnd, faHourglassHalf, faHourglassStart, faHouseDamage, faHouseFlood, faHryvnia, faHumidity, faHurricane, faICursor, faIceCream, faIceSkate, faIcicles, faIcons, faIconsAlt, faIdBadge, faIdCard, faIdCardAlt, faIgloo, faImage, faImages, faInbox, faInboxIn, faInboxOut, faIndent, faIndustry, faIndustryAlt, faInfinity, faInfo, faInfoCircle, faInfoSquare, faInhaler, faIntegral, faIntersection, faInventory, faIslandTropical, faItalic, faJackOLantern, faJedi, faJoint, faJournalWhills, faKaaba, faKerning, faKey, faKeySkeleton, faKeyboard, faKeynote, faKhanda, faKidneys, faKiss, faKissBeam, faKissWinkHeart, faKite, faKiwiBird, faKnifeKitchen, faLambda, faLamp, faLandmark, faLandmarkAlt, faLanguage, faLaptop, faLaptopCode, faLaptopMedical, faLaugh, faLaughBeam, faLaughSquint, faLaughWink, faLayerGroup, faLayerMinus, faLayerPlus, faLeaf, faLeafHeart, faLeafMaple, faLeafOak, faLemon, faLessThan, faLessThanEqual, faLevelDown, faLevelDownAlt, faLevelUp, faLevelUpAlt, faLifeRing, faLightbulb, faLightbulbDollar, faLightbulbExclamation, faLightbulbOn, faLightbulbSlash, faLightsHoliday, faLineColumns, faLineHeight, faLink, faLips, faLiraSign, faList, faListAlt, faListOl, faListUl, faLocation, faLocationArrow, faLocationCircle, faLocationSlash, faLock, faLockAlt, faLockOpen, faLockOpenAlt, faLongArrowAltDown, faLongArrowAltLeft, faLongArrowAltRight, faLongArrowAltUp, faLongArrowDown, faLongArrowLeft, faLongArrowRight, faLongArrowUp, faLoveseat, faLowVision, faLuchador, faLuggageCart, faLungs, faMace, faMagic, faMagnet, faMailBulk, faMailbox, faMale, faMandolin, faMap, faMapMarked, faMapMarkedAlt, faMapMarker, faMapMarkerAlt, faMapMarkerAltSlash, faMapMarkerCheck, faMapMarkerEdit, faMapMarkerExclamation, faMapMarkerMinus, faMapMarkerPlus, faMapMarkerQuestion, faMapMarkerSlash, faMapMarkerSmile, faMapMarkerTimes, faMapPin, faMapSigns, faMarker, faMars, faMarsDouble, faMarsStroke, faMarsStrokeH, faMarsStrokeV, faMask, faMeat, faMedal, faMedkit, faMegaphone, faMeh, faMehBlank, faMehRollingEyes, faMemory, faMenorah, faMercury, faMeteor, faMicrochip, faMicrophone, faMicrophoneAlt, faMicrophoneAltSlash, faMicrophoneSlash, faMicroscope, faMindShare, faMinus, faMinusCircle, faMinusHexagon, faMinusOctagon, faMinusSquare, faMistletoe, faMitten, faMobile, faMobileAlt, faMobileAndroid, faMobileAndroidAlt, faMoneyBill, faMoneyBillAlt, faMoneyBillWave, faMoneyBillWaveAlt, faMoneyCheck, faMoneyCheckAlt, faMoneyCheckEdit, faMoneyCheckEditAlt, faMonitorHeartRate, faMonkey, faMonument, faMoon, faMoonCloud, faMoonStars, faMortarPestle, faMosque, faMotorcycle, faMountain, faMountains, faMousePointer, faMug, faMugHot, faMugMarshmallows, faMugTea, faMusic, faNarwhal, faNetworkWired, faNeuter, faNewspaper, faNotEqual, faNotesMedical, faObjectGroup, faObjectUngroup, faOctagon, faOilCan, faOilTemp, faOm, faOmega, faOrnament, faOtter, faOutdent, faOverline, faPageBreak, faPager, faPaintBrush, faPaintBrushAlt, faPaintRoller, faPalette, faPallet, faPalletAlt, faPaperPlane, faPaperclip, faParachuteBox, faParagraph, faParagraphRtl, faParking, faParkingCircle, faParkingCircleSlash, faParkingSlash, faPassport, faPastafarianism, faPaste, faPause, faPauseCircle, faPaw, faPawAlt, faPawClaws, faPeace, faPegasus, faPen, faPenAlt, faPenFancy, faPenNib, faPenSquare, faPencil, faPencilAlt, faPencilPaintbrush, faPencilRuler, faPennant, faPeopleCarry, faPepperHot, faPercent, faPercentage, faPersonBooth, faPersonCarry, faPersonDolly, faPersonDollyEmpty, faPersonSign, faPhone, faPhoneAlt, faPhoneLaptop, faPhoneOffice, faPhonePlus, faPhoneSlash, faPhoneSquare, faPhoneSquareAlt, faPhoneVolume, faPhotoVideo, faPi, faPie, faPig, faPiggyBank, faPills, faPizza, faPizzaSlice, faPlaceOfWorship, faPlane, faPlaneAlt, faPlaneArrival, faPlaneDeparture, faPlay, faPlayCircle, faPlug, faPlus, faPlusCircle, faPlusHexagon, faPlusOctagon, faPlusSquare, faPodcast, faPodium, faPodiumStar, faPoll, faPollH, faPollPeople, faPoo, faPooStorm, faPoop, faPopcorn, faPortrait, faPoundSign, faPowerOff, faPray, faPrayingHands, faPrescription, faPrescriptionBottle, faPrescriptionBottleAlt, faPresentation, faPrint, faPrintSearch, faPrintSlash, faProcedures, faProjectDiagram, faPumpkin, faPuzzlePiece, faQrcode, faQuestion, faQuestionCircle, faQuestionSquare, faQuidditch, faQuoteLeft, faQuoteRight, faQuran, faRabbit, faRabbitFast, faRacquet, faRadiation, faRadiationAlt, faRainbow, faRaindrops, faRam, faRampLoading, faRandom, faReceipt, faRectangleLandscape, faRectanglePortrait, faRectangleWide, faRecycle, faRedo, faRedoAlt, faRegistered, faRemoveFormat, faRepeat, faRepeat1, faRepeat1Alt, faRepeatAlt, faReply, faReplyAll, faRepublican, faRestroom, faRetweet, faRetweetAlt, faRibbon, faRing, faRingsWedding, faRoad, faRobot, faRocket, faRoute, faRouteHighway, faRouteInterstate, faRss, faRssSquare, faRubleSign, faRuler, faRulerCombined, faRulerHorizontal, faRulerTriangle, faRulerVertical, faRunning, faRupeeSign, faRv, faSack, faSackDollar, faSadCry, faSadTear, faSalad, faSandwich, faSatellite, faSatelliteDish, faSausage, faSave, faScalpel, faScalpelPath, faScanner, faScannerKeyboard, faScannerTouchscreen, faScarecrow, faScarf, faSchool, faScrewdriver, faScroll, faScrollOld, faScrubber, faScythe, faSdCard, faSearch, faSearchDollar, faSearchLocation, faSearchMinus, faSearchPlus, faSeedling, faSendBack, faSendBackward, faServer, faShapes, faShare, faShareAll, faShareAlt, faShareAltSquare, faShareSquare, faSheep, faShekelSign, faShield, faShieldAlt, faShieldCheck, faShieldCross, faShip, faShippingFast, faShippingTimed, faShishKebab, faShoePrints, faShoppingBag, faShoppingBasket, faShoppingCart, faShovel, faShovelSnow, faShower, faShredder, faShuttleVan, faShuttlecock, faSickle, faSigma, faSign, faSignIn, faSignInAlt, faSignLanguage, faSignOut, faSignOutAlt, faSignal, faSignal1, faSignal2, faSignal3, faSignal4, faSignalAlt, faSignalAlt1, faSignalAlt2, faSignalAlt3, faSignalAltSlash, faSignalSlash, faSignature, faSimCard, faSitemap, faSkating, faSkeleton, faSkiJump, faSkiLift, faSkiing, faSkiingNordic, faSkull, faSkullCrossbones, faSlash, faSledding, faSleigh, faSlidersH, faSlidersHSquare, faSlidersV, faSlidersVSquare, faSmile, faSmileBeam, faSmilePlus, faSmileWink, faSmog, faSmoke, faSmoking, faSmokingBan, faSms, faSnake, faSnooze, faSnowBlowing, faSnowboarding, faSnowflake, faSnowflakes, faSnowman, faSnowmobile, faSnowplow, faSocks, faSolarPanel, faSort, faSortAlphaDown, faSortAlphaDownAlt, faSortAlphaUp, faSortAlphaUpAlt, faSortAlt, faSortAmountDown, faSortAmountDownAlt, faSortAmountUp, faSortAmountUpAlt, faSortDown, faSortNumericDown, faSortNumericDownAlt, faSortNumericUp, faSortNumericUpAlt, faSortShapesDown, faSortShapesDownAlt, faSortShapesUp, faSortShapesUpAlt, faSortSizeDown, faSortSizeDownAlt, faSortSizeUp, faSortSizeUpAlt, faSortUp, faSoup, faSpa, faSpaceShuttle, faSpade, faSparkles, faSpellCheck, faSpider, faSpiderBlackWidow, faSpiderWeb, faSpinner, faSpinnerThird, faSplotch, faSprayCan, faSquare, faSquareFull, faSquareRoot, faSquareRootAlt, faSquirrel, faStaff, faStamp, faStar, faStarAndCrescent, faStarChristmas, faStarExclamation, faStarHalf, faStarHalfAlt, faStarOfDavid, faStarOfLife, faStars, faSteak, faSteeringWheel, faStepBackward, faStepForward, faStethoscope, faStickyNote, faStocking, faStomach, faStop, faStopCircle, faStopwatch, faStore, faStoreAlt, faStream, faStreetView, faStretcher, faStrikethrough, faStroopwafel, faSubscript, faSubway, faSuitcase, faSuitcaseRolling, faSun, faSunCloud, faSunDust, faSunHaze, faSunglasses, faSunrise, faSunset, faSuperscript, faSurprise, faSwatchbook, faSwimmer, faSwimmingPool, faSword, faSwords, faSynagogue, faSync, faSyncAlt, faSyringe, faTable, faTableTennis, faTablet, faTabletAlt, faTabletAndroid, faTabletAndroidAlt, faTabletRugged, faTablets, faTachometer, faTachometerAlt, faTachometerAltAverage, faTachometerAltFast, faTachometerAltFastest, faTachometerAltSlow, faTachometerAltSlowest, faTachometerAverage, faTachometerFast, faTachometerFastest, faTachometerSlow, faTachometerSlowest, faTaco, faTag, faTags, faTally, faTanakh, faTape, faTasks, faTasksAlt, faTaxi, faTeeth, faTeethOpen, faTemperatureFrigid, faTemperatureHigh, faTemperatureHot, faTemperatureLow, faTenge, faTennisBall, faTerminal, faText, faTextHeight, faTextSize, faTextWidth, faTh, faThLarge, faThList, faTheaterMasks, faThermometer, faThermometerEmpty, faThermometerFull, faThermometerHalf, faThermometerQuarter, faThermometerThreeQuarters, faTheta, faThumbsDown, faThumbsUp, faThumbtack, faThunderstorm, faThunderstormMoon, faThunderstormSun, faTicket, faTicketAlt, faTilde, faTimes, faTimesCircle, faTimesHexagon, faTimesOctagon, faTimesSquare, faTint, faTintSlash, faTire, faTireFlat, faTirePressureWarning, faTireRugged, faTired, faToggleOff, faToggleOn, faToilet, faToiletPaper, faToiletPaperAlt, faTombstone, faTombstoneAlt, faToolbox, faTools, faTooth, faToothbrush, faTorah, faToriiGate, faTornado, faTractor, faTrademark, faTrafficCone, faTrafficLight, faTrafficLightGo, faTrafficLightSlow, faTrafficLightStop, faTrain, faTram, faTransgender, faTransgenderAlt, faTrash, faTrashAlt, faTrashRestore, faTrashRestoreAlt, faTrashUndo, faTrashUndoAlt, faTreasureChest, faTree, faTreeAlt, faTreeChristmas, faTreeDecorated, faTreeLarge, faTreePalm, faTrees, faTriangle, faTrophy, faTrophyAlt, faTruck, faTruckContainer, faTruckCouch, faTruckLoading, faTruckMonster, faTruckMoving, faTruckPickup, faTruckPlow, faTruckRamp, faTshirt, faTty, faTurkey, faTurtle, faTv, faTvRetro, faUmbrella, faUmbrellaBeach, faUnderline, faUndo, faUndoAlt, faUnicorn, faUnion, faUniversalAccess, faUniversity, faUnlink, faUnlock, faUnlockAlt, faUpload, faUsdCircle, faUsdSquare, faUser, faUserAlt, faUserAltSlash, faUserAstronaut, faUserChart, faUserCheck, faUserCircle, faUserClock, faUserCog, faUserCrown, faUserEdit, faUserFriends, faUserGraduate, faUserHardHat, faUserHeadset, faUserInjured, faUserLock, faUserMd, faUserMdChat, faUserMinus, faUserNinja, faUserNurse, faUserPlus, faUserSecret, faUserShield, faUserSlash, faUserTag, faUserTie, faUserTimes, faUsers, faUsersClass, faUsersCog, faUsersCrown, faUsersMedical, faUtensilFork, faUtensilKnife, faUtensilSpoon, faUtensils, faUtensilsAlt, faValueAbsolute, faVectorSquare, faVenus, faVenusDouble, faVenusMars, faVial, faVials, faVideo, faVideoPlus, faVideoSlash, faVihara, faVoicemail, faVolcano, faVolleyballBall, faVolume, faVolumeDown, faVolumeMute, faVolumeOff, faVolumeSlash, faVolumeUp, faVoteNay, faVoteYea, faVrCardboard, faWalker, faWalking, faWallet, faWand, faWandMagic, faWarehouse, faWarehouseAlt, faWasher, faWatch, faWatchFitness, faWater, faWaterLower, faWaterRise, faWaveSine, faWaveSquare, faWaveTriangle, faWebcam, faWebcamSlash, faWeight, faWeightHanging, faWhale, faWheat, faWheelchair, faWhistle, faWifi, faWifi1, faWifi2, faWifiSlash, faWind, faWindTurbine, faWindWarning, faWindow, faWindowAlt, faWindowClose, faWindowMaximize, faWindowMinimize, faWindowRestore, faWindsock, faWineBottle, faWineGlass, faWineGlassAlt, faWonSign, faWreath, faWrench, faXRay, faYenSign, faYinYang };
//     "#;
//     parse(source).unwrap();
//   }

//   #[test]
//   fn empty_export() {
//     let source = r#"
//       export {};
//     "#;

//     let analysis = parse(source).unwrap();
//     assert_eq!(analysis.imports.len(), 0);
//     assert_eq!(analysis.exports.len(), 0);
//   }
}
