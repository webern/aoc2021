use crate::day5::{Line, Point};

#[allow(unused)]
pub(super) fn test_data() -> Vec<Line> {
    parse(TEST)
}

pub(super) fn input_data() -> Vec<Line> {
    parse(INPUT_DATA)
}

fn parse(s: &str) -> Vec<Line> {
    s.lines().map(|line| parse_line(line)).collect()
}

fn parse_line(s: &str) -> Line {
    let mut iter = s.split("-");
    let a = parse_point(iter.next().unwrap());
    let b = parse_point(iter.next().unwrap());
    Line { a, b }
}

fn parse_point(s: &str) -> Point {
    let yuck = s.replace(' ', "").replace('>', "");
    let mut iter = yuck.split(",");
    let x = iter.next().unwrap().parse::<usize>().unwrap();
    let y = iter.next().unwrap().parse::<usize>().unwrap();
    Point { x, y }
}

#[test]
fn test_parse() {
    let actual = parse_point("8,0 ");
    assert_eq!(Point { x: 8, y: 0 }, actual);
    let actual = parse_point("> 7,4");
    assert_eq!(Point { x: 7, y: 4 }, actual);
    let actual = parse_line("6,4 -> 2,0");
    assert_eq!(
        Line {
            a: Point { x: 6, y: 4 },
            b: Point { x: 2, y: 0 },
        },
        actual
    );
    let actual = parse(
        r#"3,4 -> 1,4
0,0 -> 8,8"#,
    );
    assert_eq!(2, actual.len());
    assert_eq!(
        Line {
            a: Point { x: 3, y: 4 },
            b: Point { x: 1, y: 4 },
        },
        *actual.get(0).unwrap()
    );
    assert_eq!(
        Line {
            a: Point { x: 0, y: 0 },
            b: Point { x: 8, y: 8 },
        },
        *actual.get(1).unwrap()
    );
}

#[allow(unused)]
pub const TEST: &str = r#"0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2"#;

pub const INPUT_DATA: &str = r#"578,391 -> 578,322
274,585 -> 651,962
482,348 -> 294,348
682,514 -> 367,829
180,243 -> 800,863
850,828 -> 850,98
698,439 -> 460,677
518,379 -> 518,176
486,437 -> 486,640
730,420 -> 374,420
738,726 -> 632,726
48,959 -> 468,539
246,526 -> 246,174
490,438 -> 291,239
975,116 -> 272,116
695,883 -> 476,883
129,393 -> 129,300
658,556 -> 658,436
860,777 -> 860,365
229,321 -> 422,514
814,312 -> 752,312
886,103 -> 783,206
860,786 -> 701,945
551,789 -> 479,789
103,685 -> 687,685
649,395 -> 758,395
48,233 -> 48,677
385,22 -> 385,120
731,546 -> 731,463
570,507 -> 930,507
92,288 -> 780,976
270,622 -> 270,231
791,76 -> 791,769
926,60 -> 25,961
972,986 -> 47,61
382,601 -> 345,638
536,122 -> 536,822
963,864 -> 532,433
590,550 -> 590,221
768,744 -> 768,981
842,129 -> 842,65
521,548 -> 777,548
897,410 -> 773,410
433,738 -> 802,369
498,815 -> 498,874
93,905 -> 837,161
552,281 -> 552,491
274,82 -> 274,760
566,398 -> 78,886
602,654 -> 256,654
204,816 -> 818,202
488,265 -> 330,107
359,620 -> 71,332
915,133 -> 915,689
698,119 -> 316,501
347,25 -> 415,25
835,902 -> 835,65
900,539 -> 474,113
693,809 -> 245,809
16,32 -> 964,980
177,94 -> 637,554
824,455 -> 346,455
800,893 -> 264,893
109,342 -> 109,676
204,630 -> 281,630
798,930 -> 154,930
287,688 -> 287,106
67,641 -> 970,641
988,908 -> 362,282
411,949 -> 781,949
43,356 -> 187,356
331,848 -> 178,695
513,658 -> 513,763
313,250 -> 605,542
514,552 -> 185,223
652,726 -> 869,726
291,590 -> 291,969
861,808 -> 861,379
842,170 -> 842,928
570,166 -> 570,285
764,439 -> 764,486
200,806 -> 910,806
199,200 -> 876,200
323,474 -> 323,433
258,426 -> 258,808
568,575 -> 568,34
979,982 -> 12,15
424,534 -> 649,759
763,710 -> 147,94
339,232 -> 832,232
10,19 -> 450,19
241,846 -> 45,650
727,990 -> 727,273
596,555 -> 781,370
431,950 -> 431,627
259,415 -> 259,358
803,236 -> 515,236
239,735 -> 603,735
982,377 -> 982,581
779,221 -> 405,595
517,288 -> 414,288
376,688 -> 376,892
450,300 -> 293,143
147,217 -> 871,217
40,144 -> 156,144
913,873 -> 632,592
14,415 -> 274,155
21,987 -> 950,58
979,960 -> 37,18
50,903 -> 890,63
32,523 -> 426,523
625,491 -> 625,692
46,47 -> 899,900
226,633 -> 226,318
24,136 -> 24,693
870,675 -> 850,675
883,862 -> 883,421
581,97 -> 219,97
537,743 -> 434,743
977,77 -> 957,77
139,720 -> 139,403
248,14 -> 394,14
88,55 -> 866,833
562,652 -> 987,227
265,54 -> 958,747
322,161 -> 322,573
574,236 -> 311,236
919,393 -> 919,587
604,906 -> 604,156
691,468 -> 448,225
948,167 -> 948,516
218,238 -> 218,92
989,229 -> 99,229
384,481 -> 384,15
618,681 -> 618,815
292,956 -> 922,326
599,967 -> 599,250
418,648 -> 961,105
120,791 -> 196,791
779,559 -> 582,362
953,941 -> 35,23
508,934 -> 340,934
707,752 -> 915,752
514,958 -> 514,926
15,945 -> 826,134
433,921 -> 821,533
378,80 -> 378,407
76,957 -> 858,175
791,617 -> 662,488
891,897 -> 52,58
786,841 -> 786,973
774,799 -> 348,373
812,48 -> 40,820
57,749 -> 57,767
68,750 -> 68,891
774,920 -> 156,302
598,400 -> 116,882
34,285 -> 856,285
14,473 -> 14,134
594,877 -> 594,333
38,989 -> 964,63
631,209 -> 631,121
45,296 -> 468,296
708,904 -> 11,904
960,20 -> 99,881
412,557 -> 345,557
29,389 -> 504,864
397,713 -> 251,713
350,548 -> 350,61
134,610 -> 579,165
675,947 -> 789,947
12,986 -> 949,49
765,601 -> 765,627
817,701 -> 817,305
508,532 -> 538,502
383,136 -> 383,700
771,549 -> 443,549
283,134 -> 987,838
171,855 -> 171,248
841,858 -> 620,858
512,26 -> 912,26
425,39 -> 180,39
116,279 -> 121,279
282,482 -> 282,939
58,937 -> 980,15
376,641 -> 376,503
548,17 -> 249,17
730,411 -> 427,714
600,73 -> 541,73
656,619 -> 656,810
467,237 -> 467,255
694,946 -> 446,946
168,646 -> 395,646
731,265 -> 731,20
12,172 -> 286,446
385,762 -> 244,903
941,366 -> 941,807
125,383 -> 367,383
341,177 -> 341,809
544,830 -> 544,192
801,943 -> 731,873
862,436 -> 950,436
484,422 -> 484,267
883,155 -> 328,155
499,321 -> 499,449
128,310 -> 778,960
788,571 -> 788,795
523,765 -> 319,765
267,928 -> 267,665
227,829 -> 797,829
96,972 -> 733,335
178,364 -> 178,425
793,201 -> 848,201
975,242 -> 497,720
673,242 -> 513,242
199,163 -> 862,826
988,51 -> 225,814
631,928 -> 631,567
22,474 -> 854,474
717,607 -> 717,514
436,753 -> 905,753
581,343 -> 581,641
128,912 -> 964,76
706,634 -> 843,634
89,826 -> 89,667
766,268 -> 103,268
229,131 -> 229,138
138,112 -> 388,362
434,117 -> 434,387
313,746 -> 313,941
517,944 -> 145,944
611,945 -> 611,872
400,869 -> 329,869
444,701 -> 700,957
894,975 -> 426,975
722,544 -> 722,55
692,927 -> 692,874
451,211 -> 145,211
562,850 -> 562,252
833,154 -> 703,284
700,911 -> 700,738
32,982 -> 891,123
512,512 -> 403,512
444,963 -> 40,559
866,53 -> 866,733
395,90 -> 603,90
781,175 -> 506,175
649,569 -> 210,130
861,926 -> 79,144
160,953 -> 735,953
138,837 -> 138,166
659,683 -> 659,656
198,587 -> 725,60
290,36 -> 785,36
481,228 -> 785,532
721,152 -> 192,681
162,445 -> 162,476
286,93 -> 286,611
882,393 -> 770,393
194,703 -> 194,714
172,505 -> 153,524
989,986 -> 48,45
946,334 -> 946,864
543,48 -> 485,48
276,520 -> 184,612
879,488 -> 665,488
706,312 -> 706,300
859,958 -> 533,958
345,591 -> 345,685
201,734 -> 310,734
610,781 -> 610,250
25,702 -> 25,470
127,802 -> 46,802
899,330 -> 899,942
266,118 -> 266,978
871,535 -> 871,230
346,290 -> 346,138
411,171 -> 911,671
104,427 -> 500,31
531,115 -> 531,861
164,699 -> 529,699
215,560 -> 97,442
331,323 -> 331,321
74,969 -> 74,57
894,743 -> 739,588
913,895 -> 160,895
868,291 -> 868,987
913,390 -> 913,144
548,812 -> 889,812
978,819 -> 673,514
989,130 -> 989,589
986,12 -> 10,988
48,18 -> 974,944
511,336 -> 736,111
61,609 -> 61,742
536,650 -> 773,650
924,691 -> 307,74
49,988 -> 912,125
128,692 -> 128,969
569,837 -> 916,837
849,745 -> 849,105
524,926 -> 357,926
110,827 -> 661,827
911,36 -> 49,898
967,15 -> 23,959
969,166 -> 155,980
204,684 -> 805,83
230,960 -> 230,556
309,718 -> 522,931
121,208 -> 121,443
733,797 -> 710,820
813,780 -> 813,909
154,97 -> 375,318
117,916 -> 984,49
573,525 -> 573,980
442,636 -> 383,695
938,21 -> 938,50
38,672 -> 196,672
52,829 -> 52,835
661,278 -> 157,782
525,347 -> 285,347
339,468 -> 339,42
10,20 -> 976,986
953,812 -> 445,304
328,327 -> 711,327
750,820 -> 750,172
244,935 -> 244,360
842,36 -> 181,697
559,730 -> 320,730
149,510 -> 524,510
713,913 -> 262,462
703,957 -> 643,957
170,930 -> 767,930
804,259 -> 635,90
117,948 -> 932,133
263,806 -> 981,806
307,665 -> 307,743
697,164 -> 665,132
589,568 -> 872,285
865,189 -> 417,637
77,76 -> 951,950
546,350 -> 769,350
533,479 -> 566,446
689,79 -> 689,417
132,666 -> 888,666
661,88 -> 155,88
93,27 -> 852,786
536,366 -> 815,366
97,649 -> 97,214
50,784 -> 691,143
523,687 -> 523,881
720,825 -> 865,825
103,985 -> 939,149
135,94 -> 91,50
959,26 -> 18,967
391,617 -> 391,147
522,103 -> 522,202
161,774 -> 742,193
125,291 -> 125,513
449,436 -> 726,436
438,127 -> 499,66
804,577 -> 804,385
714,112 -> 714,90
111,184 -> 907,980
218,209 -> 53,209
343,949 -> 73,679
50,205 -> 828,983
416,664 -> 416,213
300,902 -> 300,137
563,366 -> 307,366
302,750 -> 572,750
436,59 -> 512,59
363,299 -> 363,471
969,988 -> 10,29
15,349 -> 15,424
855,231 -> 855,241
93,771 -> 540,324
360,363 -> 360,481
890,391 -> 890,824
603,916 -> 780,916
686,776 -> 165,255
905,64 -> 37,932
937,607 -> 937,846
634,108 -> 971,108
118,419 -> 292,419
724,241 -> 724,663
118,327 -> 688,327
728,316 -> 507,316
824,652 -> 744,652
985,72 -> 93,964
791,652 -> 791,621
475,488 -> 475,448
289,386 -> 648,386
833,925 -> 120,925
323,813 -> 652,813
631,615 -> 248,615
191,222 -> 603,634
445,322 -> 964,322
238,672 -> 142,672
170,370 -> 439,370
158,77 -> 491,410
165,737 -> 816,737
420,957 -> 709,668
936,283 -> 681,283
76,781 -> 291,781
197,575 -> 656,116
577,746 -> 577,748
435,198 -> 435,803
787,623 -> 787,153
476,176 -> 670,176
107,581 -> 107,167
575,495 -> 186,106
283,760 -> 19,760
910,483 -> 871,483
550,99 -> 550,94
338,522 -> 589,522
856,435 -> 856,388
890,380 -> 392,878
524,885 -> 315,676
23,34 -> 769,780
686,647 -> 545,647
760,442 -> 564,246
535,264 -> 61,264
709,168 -> 709,33
89,230 -> 604,230
476,558 -> 82,558
905,48 -> 294,48
695,882 -> 695,153
785,716 -> 94,716
390,990 -> 390,757
775,699 -> 783,699
965,126 -> 425,126
572,45 -> 482,45
399,391 -> 399,827
310,660 -> 947,23
418,813 -> 72,467
292,911 -> 506,697
177,685 -> 177,100
749,294 -> 749,927
304,832 -> 833,303
237,759 -> 923,73
834,95 -> 15,914
233,99 -> 822,99
462,841 -> 462,845
968,70 -> 815,70
820,565 -> 241,565
849,469 -> 648,670
10,825 -> 906,825
105,105 -> 526,526
977,173 -> 711,173
347,66 -> 347,959
921,42 -> 41,42
100,264 -> 100,580
608,211 -> 166,653
826,171 -> 509,171
346,541 -> 802,85
351,70 -> 872,70
649,79 -> 590,79
974,31 -> 24,981
876,145 -> 227,794
855,903 -> 855,891
621,734 -> 621,930
190,184 -> 727,721
210,855 -> 564,855
612,919 -> 612,628
258,851 -> 573,851
842,85 -> 140,787
252,312 -> 252,17
82,352 -> 135,352
365,583 -> 854,583
939,666 -> 525,252
257,481 -> 257,591
382,725 -> 382,786
326,111 -> 38,399
476,480 -> 476,544
592,49 -> 592,473
626,748 -> 626,477
612,574 -> 19,574
638,734 -> 604,734
240,794 -> 770,794
598,931 -> 37,370
666,559 -> 573,559
208,337 -> 784,913
24,17 -> 988,981
324,267 -> 332,267
233,589 -> 300,589
53,46 -> 986,979
193,649 -> 243,649
873,600 -> 873,618
461,102 -> 638,102
468,574 -> 507,535
261,521 -> 658,521
540,234 -> 769,234
975,337 -> 975,478
724,982 -> 585,982
449,639 -> 449,255
47,296 -> 751,296
700,262 -> 903,262
838,833 -> 838,626
956,17 -> 24,949"#;
