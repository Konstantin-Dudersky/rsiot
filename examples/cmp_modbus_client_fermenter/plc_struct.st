TYPE SLAVE_DATA:
STRUCT
	wWordState1_Get: WORD;	(* REG 0 *)
(*
	.00: xCZDispenserSensor_Get;
	.01: xBZDispenserSensor_Get;
	.02: xFZFConverterAlarm_Get;
	.03: xFZMixerDriveUnitSwitchOn_Get;
	.04: xBunkerFConverterAlarm_Get;
	.05: xBZMixerDriveUnitSwitchOn_Get;
	.06: g_xMZSensorLevelUpper_Get;
	.07: g_xMZSensorLevelLower_Get;
	.08: g_xFZSensorLevelLower_Get;
	.09: g_xFZSensorLevelUpper_Get;
	.10: xOperatingModeManual_Get;
	.11: xOperatingModeAutomatic_Get;
	.12: xOperatingModeLoad_Get;
	.13: xClosetASSupplyDosingDrives_Get;
	.14: xClosetASTempCSCirculation_Get;
	.15: xMZMixerDriveUnitSwitchOn_Get;
 *)
	wWordState2: WORD;	(* REG 1 *)
(*
	.00: OpenWindowsResetError;
	.01: xBZCapOpen_Get;
	.02: MaskState.0;	(* xCZMixerDriveUnitSwitchOn_Get0 *)
	.03: BitVisibleNrecept;
	.04: MaskState.2;	(* xCZDispenserSwitchOn_Get *)
	.05: MaskState.3;	(* xBZDispenserSwitchOn_Get *)
	.06: BaseReceptRecord;
	.07: BitVisibleStart1;
	.08: g_BZAlarmCT;
	.09: g_CZAlarmCT;
	.10: CZCicle;
	.11: BZCicle;
	.12: BitVisibleStart;
	.13: BitVisiblePause;
	.14: BitVisibleFinish;
	.15: g_xFlagSelectMode_Get ;
 * )
	wWordCMD1: WORD;	(* REG 2 *)
(*
	.00: xMZMixerDriveUnit_Set		- âêëþ÷åíèå ìèêñåðà çîíû ñìåøèâàíèÿ
	.01: xFZMixerDriveUnit_Set			- âêëþ÷åíèå ìèêñåðà çîíû ôåðìåíòàöèè
	.02: xBZMixerDriveUnit_Set		- âêëþ÷åíèå ìèêñåðà çîíû áóíêåðà
	.03:
	.04: xTempCSHeatingElement_Set 	- âêëþ÷èòü íàãðåâàòåëü
	.05: xTempCSValveColdWater_Set 	- îòêðûòü çàäâèæêó õîëîäíîé âîäû
	.06: xTempCSCirculatingPump_Set - âêëþ÷èòü íàñîñ ðóáàøêè
	.07: SaveRecipe					- ñîõðàíèòü ðåöåïò
	.08: xColorRedCmd_Set	 		- âêëþ÷èòü êðàñíûé
	.09: xColorYellowCmd_Set	 		- âêëþ÷èòü æåëòûé
	.10: xColorGreenCmd_Set	 		- âêëþ÷èòü çåëåíûé
	.11: xBuzzerCmd_Set				- âêëþèòü çâóê
	.12: xBZDispenser_Set				- âêëþ÷åíèå äîçàòîðà ñûïó÷åãî ñûðüÿ
	.13: xCZDispenser_Set				- âêëþ÷åíèå äîçàòîðà ñóñïåíçèè
	.14: xFZDriveUnitMixSupply_Set	- âêëþ÷åíèå ïðèâîäà ïîäà÷è ñìåñè â ôåðìåíòåð
	.15: xCZMixerDriveUnit_Set 		- âêëþ÷åíèå ìèêñåðà íàêîïèòåëüíîé åìêîñòè
 *)
	iOperationModeSet: WORD;	(* REG 3 *)
	rTempCSSensorTValue_Get: REAL;		(* REG 4..5 *)
	rMZSensorCurrentRS1_Get: REAL;		(* REG 6..7 *)
	rMZSensorCurrentRS2_Get: REAL;		(* REG 8..9 *)
	rFZSensorCurrentRS3_Get: REAL;		(* REG 10..11 *)
	rCZSensorCurrentRS4_Get: REAL;		(* REG 12..13 *)
	BZTimeBetweenStartManual: WORD;	(* REG 14 *)
	BZTimePulseStartManual: WORD;	(* REG 15 *)
	CZTimeBetweenStartManual: WORD;	(* REG 16 *)
	CZTimePulseStartManual: WORD;	(* REG 17 *)
	TimeFermentH_Current: WORD;	(* REG 18 *)
	wWordError: WORD;	(* REG 19 *)
(*
	.00: error inv20
	.01: error inv22
	.02: error inv24
	.03: error connection
	.04: error îñòàíîâ äîçàòîðîâ îò çîíû ôåðìåíòàöèè
	.05: îøèáêà âåðõíåãî óðîâíÿ çîíû ôåðìåíòàöèè
	.06: îñòàíîâ äîçàòîðîâ îò çîíû ñìåøèâàíèÿ
	.07: îøèáêà âåðõíåãî óðîâíÿ çîíû ñìåøèâàíèÿ
	.08: îøèáêà äîçàòîðà ñóõèõ
	.09: îøèáêà äîçàòîðà ìîêðûõ
	.10:
	.11:
	.12: îøèáêà íåò öèðêóëÿöèè
	.15:
 *)
	Invertor20Frequency: WORD;	(* REG 20 *) (* áóíêåð *)
	Invertor22Frequency: WORD;	(* REG 21 *) (* ñìåñèòåëü *)
	Invertor24Frequency: WORD;	(* REG 22 *) (* ôåðìåíòåð *)
	TimeFermentM_Current: WORD;	(* REG 23 *)
	motodays: DWORD;	(* REG 24..25 *)
	motohours: DWORD;	(* REG 26..27 *)
	motomins: DWORD;	(* REG 28..29 *)
	CZDispenserL: REAL;		(* REG 30..31 *)
	FZMixerPart1N: WORD;	(* REG 32 *)
	FZMixerPart2N: WORD;	(* REG 33 *)
	FZMixerPart3N: WORD;	(* REG 34 *)
	FZMixerPart4N: WORD;	(* REG 35 *)
	MZMixerPart1N: WORD;	(* REG 36 *)
	MZMixerPart2N: WORD;	(* REG 37 *)
	MZMixerPart3N: WORD;	(* REG 38 *)
	MZMixerPart4N: WORD;	(* REG 39 *)
	TimeFermentH_SP: WORD;	(* REG 40 *)
	TimeFermentM_SP: WORD;	(* REG 41 *)
	NumberFormation_Get: WORD;	(* REG 42 *)
	AllMixingSec: WORD;	(* REG 43 *)
	CZDispenserH: WORD;	(* REG 44 *)
	CZDispenserM: WORD;	(* REG 45 *)
	NumberRecept: WORD;	(* REG 46 *)
	CommError: WORD;	(* REG 47 *)
	BZDispenserH: WORD;	(* REG 48 *)
	BZDispenserM: WORD;	(* REG 49 *)
	BZDispenserL: REAL;		(* REG 50..51 *)

	FZMixerPart1H: WORD;	(* REG 52 *)
	FZMixerPart2H: WORD;	(* REG 53 *)
	FZMixerPart3H: WORD;	(* REG 54 *)
	FZMixerPart4H: WORD;	(* REG 55 *)
	FZMixerPart1M: WORD;	(* REG 56 *)
	FZMixerPart2M: WORD;	(* REG 57 *)
	FZMixerPart3M: WORD;	(* REG 58 *)
	FZMixerPart4M: WORD;	(* REG 59 *)

	FZTempPart1H: WORD;	(* REG 60 *)
	FZTempPart2H: WORD;	(* REG 61 *)
	FZTempPart3H: WORD;	(* REG 62 *)
	FZTempPart4H: WORD;	(* REG 63 *)
	FZTempPart1M: WORD;	(* REG 64 *)
	FZTempPart2M: WORD;	(* REG 65 *)
	FZTempPart3M: WORD;	(* REG 66 *)
	FZTempPart4M: WORD;	(* REG 67 *)
	FZTempPart1C: WORD;	(* REG 68 *)
	FZTempPart2C: WORD;	(* REG 69 *)
	FZTempPart3C: WORD;	(* REG 70 *)
	FZTempPart4C: WORD;	(* REG 71 *)

	MZMixerPart1H: WORD;	(* REG 72 *)
	MZMixerPart2H: WORD;	(* REG 73 *)
	MZMixerPart3H: WORD;	(* REG 74 *)
	MZMixerPart4H: WORD;	(* REG 75 *)
	MZMixerPart1M: WORD;	(* REG 76 *)
	MZMixerPart2M: WORD;	(* REG 77 *)
	MZMixerPart3M: WORD;	(* REG 78 *)
	MZMixerPart4M: WORD;	(* REG 79 *)
	NScreen: WORD;	(* REG 80 *)
	wWordCMD2: WORD;	(* REG 81 *)
(*
	00: xTransitionFlag		- ôëàã ïåðåõîäà íà äðóãîé ýêðàí
	01: xSoftStop			- ñîôòîâûé ñòîï
	02: ShowWindowReturn
	03: ShowWindowInstruct
	04:	ResetPauseSound;
	05:	VisiblePauseSound;
	06: ResetError;
	07: ResetSoundError;
	10: SystemSoundOff
	12: xBunkerCupOpen
	13: gAM_xStart_Set	- ñåé÷àñ êîìàíäà ñòàðò
	14: gAM_xPause_Set 	- ñåé÷àñ êîìàíäà ïàóçà
	15: gAM_xFinish_Set	- ñåé÷àñ êîìàíäà ôèíèø
 *)
	TimeFermentS_Current: WORD;	(* REG 82 *)
	NumberLoopFerment: WORD := 1;	(* REG 83 *)
	HeatingElementON: REAL;		(* REG 84..85 *)
	HeatingElementOFF: REAL;		(* REG 86..87 *)
	SPDeltaTemp: REAL;		(* REG 88..89 *)
	PercentBZDispenser: REAL;		(* REG 90..91 *)
	PercentCZDispenser: REAL;		(* REG 92..93 *)
	PosXFZ: INT;		(* REG 94 *)
	PosYFZ: INT;		(* REG 95 *)
	PosXMZ: INT;		(* REG 96 *)
	PosYMZ: INT;		(* REG 97 *)
	PosXFZTemp: INT;		(* REG 98 *)
	PosYFZTemp: INT;		(* REG 99 *)
	CZDoza: REAL;		(* REG 100..101 *)
	BZDoza: REAL;		(* REG 102..103 *)
	CoolingElementON: REAL;		(* REG 104..105 *)
	CoolingElementOFF: REAL;		(* REG 106..107 *)
	SPTimerPause: WORD;	(* REG 108 *)
	Invertor20Error: WORD;	(* REG 109 *)
	CurrentTimerPause: DWORD;	(* REG 110..111 *)
	Invertor22Error: WORD;	(* REG 112 *)
	Invertor24Error: WORD;	(* REG 113 *)
	CommErrorState: WORD;	(* REG 114 *)
(*
 *)
	DelaySensors: WORD;	(* REG 115 *)
	spErrorCSTemp: REAL;		(* REG 116..117 *)
	spErrorHeater: REAL;		(* REG 118..119 *)
	spErrorPump: REAL;		(* REG 120..121 *)
	HardBitPLC: WORD;	(* REG 122 *)
	HardBitPanel: WORD;	(* REG 123 *)
	BZTimeBetweenStartAuto: WORD;	(* REG 124 *)
	CZTimeBetweenStartAuto: WORD;	(* REG 125 *)
	wWordState3: WORD;	(* REG 126 *)
(*
01:
02:
03:
04:
05:
06:
07:
08:
09:
10:
11:
12:
13:
14:
15:	bWindowRecept0
 *)
	wSP_BZErrorSensor: WORD := 3;	(* REG 127 *)
	wSP_CZErrorSensor: WORD := 3;	(* REG 128 *)
END_STRUCT
END_TYPE