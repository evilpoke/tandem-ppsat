

  //////////////////////////// Stack structures

  // substitute with 0-based size value

  // 4: 4               ->  4
  // variablesnumberusize: 4usize     ->  4usize

  // clausesofpartyA: 6                ->  6
  // clausesofpartyA: 6usize           ->  6usize

  // clausesofpartyB: 6                ->  6
  // clausesofpartyB: 6usize           ->  6usize

  // totalclauses:    12                    ->  12
  // totalclausesu:    12usize              ->  12usize
 
  // stacksize:       255            -> 255
  
  
  enum IsDummyStorage { 
    RealValue,
    Dummy,
    DummyPrime
  }
  
  
  
  struct StorageType {
    f: ( ([bool;4],[bool;4]) , Formula ),
    specialflag: IsDummyStorage
  
    //.... other stuff possible here
    
  }
  
  struct StackState {
    index: usize,
    arrayacc: [StorageType; 255],
  }
  

  ///////////////////// Stack operations  
  
  
  fn condpush( cond: bool, st: StorageType, stackstate: StackState ) -> StackState {
  
  if (cond == true) {
  
      let idx = stackstate.index;
  
      let mut arrayacc = stackstate.arrayacc;
      let v = stackstate.index;
      arrayacc[v] = st;
      let newv = stackstate.index + 1usize;
  
      let newstackstate = StackState {
          index: newv, //idx+1usize,
          arrayacc: arrayacc
      };
      newstackstate
  }
  else {
      stackstate
  }
  }
  
  
  fn condpop( cond: bool, stackstate: StackState, dummyvariableemptystack: StorageType, dummyvariablecondfalse: StorageType) -> (StorageType,StackState) {
  
  if (cond == true){
  
      let idx = stackstate.index;
      
      if (idx == 0usize) {
      
          (dummyvariableemptystack, stackstate)
      
      } else {
      
          let arrayacc = stackstate.arrayacc;
          let v = idx - 1usize;
          let outvalue = arrayacc[1usize];  //idx-1usize];
          
          let newstackstate = StackState {
              index: 1usize, // idx-1usize,
              arrayacc: arrayacc
          };
          
          (outvalue, newstackstate)
      }
  }
  else {
      (dummyvariablecondfalse, stackstate)
  }
  
  
  }
  
  
  
  
  
  ////////////////////// SAT Structures
  
  
  struct Clause {
  matP: [bool ;4],
  matN: [bool ;4],
  Nl: usize,
  
  }
  
  
  enum CheckCode { 
  Zero,
  One,
  Two
  }
  
  enum Sol {
  Sat,
  Unsat,
  Unknown
  }
  
  
  struct Formula {
  clauses: [Clause; 12],
  alive: [bool; 12],
  }
  
  ///////////////////////////////// SAT Operations
  
  fn generateEmptyClause() -> Clause {
  
  Clause { matP: [false; 4] , matN: [false; 4] ,  Nl: 0usize}
  
  }
  pub fn generateFormula(
      a: (
        [ ([bool; 4],[bool; 4])  ;6  ]  ,  
        [usize; 6]
    ) ,
      b: (
        [ ([bool; 4],[bool; 4])  ;6  ]  ,  
        [usize; 6]
      )
  ) -> Formula {
  
  let mut matP = [true ;4];
  let mut matN = [true ;4];
  let mut Nl = 0usize;
  let mut newclause = generateEmptyClause();
  
  let mut allclauses = [newclause; 12];
  
  // first
  for i in 0usize..6usize {
    
    matP = ((a.0)[i]).0;
    matN = ((a.0)[i]).1;
    Nl = (a.1)[i];
  
    newclause = Clause {matP: matP, matN: matN, Nl: Nl};
  
    allclauses[i] = newclause;
  
  }
  // second
  for i in 0usize..6usize {
    matP = ((b.0)[i]).0;
    matN = ((b.0)[i]).1;
    Nl = (b.1)[i];
  
    newclause = Clause {matP: matP, matN: matN, Nl: Nl};
  
    allclauses[6usize + i] = newclause;
  
  }
  
  let allalive = giveAllTrueClauses();
  
  let formula = Formula {
    clauses: allclauses,
    alive: allalive
  };
  
  formula
  
  }
  
  fn giveAllTrueClauses() -> [bool; 12] {
  
  let mut array = [true; 12];
  
  
  array
  
  }
  
  
  /*pub fn removeC(f: Formula, j: usize) -> Formula {
  
  let clauses = f.clauses;
  let mut alives = f.alive;
  alives[j] = false;
  
  let newformula = Formula {clauses: clauses, alive: alives};
  
  newformula
  
  }*/
  
  pub fn isaunitclause(c: Clause) -> bool {
  let v = c.Nl == 1usize;
  v
  
  }
  
  
  pub fn clausecontainsl(l: ( [bool;4] , [bool;4] ) , c: Clause) -> bool {
  let mut b = false;
  
  let Pj = c.matP;
  let Nj = c.matN;
  
  let indplus = (l.0);
  let indmius = (l.1); 
  for i in 0usize..4usize {
    b = b | (Pj[i] & indplus[i]) | (Nj[i] & indmius[i]);
  }
  
  b
  }
  
  pub fn removeliteralfromclause(l: ( [bool;4] , [bool;4] ) , c: Clause ) -> Clause {
  
  let mut Pj = c.matP;
  let mut Nj = c.matN;
  
  let indplus = (l.0);
  let indmius = (l.1); 
  
  let mut Nl = c.Nl;
  
  if clausecontainsl(l,c) == true {
    Nl = Nl - 1usize;
  }
  for i in 0usize..4usize {
      
    Pj[i] = Pj[i] & (Pj[i] ^ indplus[i]);
    Nj[i] = Nj[i] & (Nj[i] ^ indmius[i]);
  }
  
  let newclause = Clause { matP: Pj , matN: Nj , Nl: Nl};
  
  newclause
  
  }
  
  pub fn phiisempty(f: Formula) -> bool {
  
  let alivearray = f.alive;
  let mut somewherefilled = false;
  for i in 0usize..12usize {
    
    somewherefilled = somewherefilled | alivearray[i];
  
  }
  
  let globallyempty = !somewherefilled;
  globallyempty
  
  }
  
  
  pub fn removeclausefromformula(f: Formula, j: usize) -> Formula {
  // remove the clause j from the formula
  let mut alivearray = f.alive;
  
  alivearray[j] = false;
  
  let newf = Formula { clauses: f.clauses, alive: alivearray};
  
  newf
  }
  
  //--------------------------------- higher level methods -----------------------
  
  
  fn check(l: ( [bool;4] , [bool;4] )  , f: Formula) ->  CheckCode {
  let b0 = phiisempty(f);
  let mut b1 = false;
  let mut Cj = generateEmptyClause();
  
  let invertetl = invertliteral(l);
  for j in 0usize..12usize {
    Cj = f.clauses[j];
    if (isaunitclause(Cj) == true) & (clausecontainsl(invertetl, Cj)==true) & (f.alive[j]==true) {
      b1 = true;
    }
  }
  let mut v = CheckCode::One;
  if b0 == true {
    v = CheckCode::Zero;
  } else {
    if b1 == true {
      v = CheckCode::One;
    } else { 
      v = CheckCode::Two;
    }
  }
  
  v
  }
  
  
  
  fn invertliteral(a: ( [bool;4] , [bool;4] )) -> ( [bool;4] , [bool;4] ) {
  
  (a.1, a.0)
  
  }
  
  
  
  
  fn propagate(a: ( [bool;4] , [bool;4] )  , f: Formula) -> Formula {
  
  let mut b0 = false;
  let mut b1 = false;
  let inverteta = invertliteral(a);
  let mut Cj = generateEmptyClause();
  let mut allclauses = [Cj; 12];
  let mut ff = f;
  for j in 0usize..12usize {
    Cj = ff.clauses[j];
    b0 = clausecontainsl(a, Cj);
    b1 = clausecontainsl(inverteta, Cj);
  
    if b0 == true {
      ff = removeclausefromformula(ff, j);
    }
    if b1 == true {
      Cj = removeliteralfromclause(inverteta, Cj);
  
      allclauses = ff.clauses;
      allclauses[j] = Cj;
  
      ff = Formula{clauses:allclauses, alive: ff.alive };
    }
  
  }
  
  ff
  
  }
  
  
  struct specialformat {
  avalue: ( [bool;4] , [bool;4] ),
  bvalue: bool
  
  }
  
  
  
  fn unitsearch(f: Formula) -> specialformat {
  
  let mut a =  ( [false; 4] , [false; 4] ) ;
  let mut b = false;
  let mut Cj = generateEmptyClause();
  for j in 0usize..12usize {
    Cj = f.clauses[j];
    if (isaunitclause(Cj)==true) & (f.alive[j]==true) {
      a = (Cj.matP, Cj.matN);  // because Cj is a single clause, its format (without the Nl number) is equal to that of a clause
      b = true;
    }
  }
  
  //b = true;
  //a = ([false; 4], [false;4]);
  //let v = ( a, 4i32, b);
  
  let v = specialformat {
    avalue : a,
    bvalue : b
  };
  
  v
  //((([true, true, true,true]) , ([true, false,true,true ])) , true)
  
  
  }
  
  
  fn decision(f:  Formula ) -> ( [bool;4] , [bool;4] ) {
  // very naive decider
  // output: an assignment 
  
  
  let mut oredclause = generateEmptyClause();
  let mut ordermatP = oredclause.matP;
  let mut ordermatN = oredclause.matN;
  
  for j in 0usize..12usize {
    let Cj = f.clauses[j];
    let Cjisalive = f.alive[j];

  
    let matP = Cj.matP;
    let matN = Cj.matN;
  
    for d in 0usize..4usize {
      ordermatP[d] = (matP[d] & Cjisalive) | ordermatP[d];
    }
  
    for d in 0usize..4usize {
      ordermatN[d] = (matN[d] & Cjisalive) | ordermatN[d];
    }
  
  }
  
  let mut pselect = 0usize;
  let mut allmatPacc = false;
  for d in 0usize..4usize {
  
    if ordermatP[d] == true {
      pselect = d;    
      allmatPacc = true;
    }
  }
  
  let mut nselect = 0usize;
  for d in 0usize..4usize {
  
    if ordermatN[d] == true {
      nselect = d;
    }
  }
  
  let zeroedclause1 = generateEmptyClause();
  let mut zeroedclause1matp = zeroedclause1.matP;   // optimierung!!
  let mut zeroedclause1matn = zeroedclause1.matN;
  
  //let mut zeroedclause2 = generateEmptyClause();
  //let mut zeroedclause2matp = zeroedclause2.matP;
  //let mut zeroedclause2matn = zeroedclause2.matN;
  
  zeroedclause1matp[pselect] = true;
  zeroedclause1matn[nselect] = true;
  
  
  let resultassignment = generateEmptyClause();
  let mut resultassignmentmatP = resultassignment.matP;
  let mut resultassignmentmatN = resultassignment.matN;
  
  if allmatPacc == true {
    resultassignmentmatP = zeroedclause1matp;
  } else {
    resultassignmentmatN = zeroedclause1matn;
  }
  
  (resultassignmentmatP, resultassignmentmatN)
  
              
  }
  
  fn first_giant_step(f: Formula, os: StackState) -> (([bool; 4], [bool; 4]), StackState) {
  
  let mut os_o = os;
  let mut a = ([true; 4] , [true;4]);
  // unit search
  let b_conflict = false;
  let v = unitsearch(f);
  let b_unit = v.bvalue;
  let a_unit = v.avalue;
  
  
  // decision
  let a_dec = decision(f);
  // cond push
  
  let valuetopush = StorageType {
    f: (a_dec, f),
    specialflag: IsDummyStorage::RealValue
  };
  
  os_o = condpush( !b_unit & !b_conflict , valuetopush, os_o );
  
  
  if b_unit == true {
    a = a_unit;
  } else {
    a = a_dec;
  } 
  
  (a,os_o)
  
  }
  
  
  fn next_giant_step(f: Formula, os:StackState, a: ([bool; 4], [bool; 4])) -> (Sol,  ([bool; 4], [bool; 4]) , StackState, Formula)  {
  
  // output: (Sol, bool, StackState, Formula) 
  
  let mut os_o = os;
  let mut newa = ([false; 4], [false;4]);
  let mut ff = f;
  
  // --- UNIT SEARCH & propagate --- 
  let sigma = check(a, f);
  let mut satresult =  Sol::Unknown;
  
  if sigma == CheckCode::Zero {
    satresult = Sol::Sat;
  }
  
  let b_conflict = sigma == CheckCode::One;
  
  //  cond: bool, stackstate: StackState, dummyvariableemptystack: StorageType, dummyvariablecondfalse: StorageType) -> (StorageType,StackState
  
  let nonsenseassignment = ([true;4], [true;4]);
  let dummyvariableemptystack = StorageType {
    f: (nonsenseassignment, f),
    specialflag: IsDummyStorage::Dummy
  };
  
  let dummyvariablecondfalse = StorageType {
    f: (nonsenseassignment, f),
    specialflag: IsDummyStorage::DummyPrime
  };
  
  //condpop( cond: bool, stackstate: StackState, dummyvariableemptystack: StorageType, dummyvariablecondfalse: StorageType) -> (StorageType,StackState) {
  
  let res1 = condpop(b_conflict, os_o, dummyvariableemptystack, dummyvariablecondfalse); 
  os_o = res1.1;
  let outvar = res1.0;
  let returntype = outvar.specialflag;
  if (returntype == IsDummyStorage::Dummy) == true {  // testing if dummyvariableemptystack
    satresult = Sol::Unsat; 
  }
  let e = outvar.f; // actually extracting the formula
  
  //if areliteralsequal(dummyvariableemptystack, outvar) { //fix this
  //  satresult = Sol.Unsat;   
  //}
  
  let a_back = e.0;
  let phi_back = e.1;
  let phi_prop = propagate(a,f);
  
  // --- SELECT PROPAGATION & push the backpropagate
  let v = unitsearch(phi_prop);
  let b_unit = v.bvalue;
  let a_unit = v.avalue;
  
  let a_dec = decision(phi_prop);
  
  let valuetopush = StorageType{
    f: (a_dec, phi_prop),
    specialflag: IsDummyStorage::RealValue
  };
  os_o = condpush( (!b_unit) & (!b_conflict) , valuetopush , os_o);
  
  
  if b_unit == true {
    newa = a_unit;
  } else {
    newa = a_dec;
  }
  
  let mut newf = phi_prop;
  if b_conflict == true {
    newa = invertliteral(a_back);
    newf = phi_back;
  }
  
  // (Sol, bool, StackState, Formula) 
  ( satresult,newa, os_o, newf)
  
  }
  
  
  
  pub fn main(a: (
                  [ ([bool; 4],[bool; 4])  ;  6  ]  ,  
                  [usize; 6]
              ) ,
            b: (
              [ ([bool; 4],[bool; 4])  ;  6  ]  ,  
                  [usize; 6]
              )
        ) -> u8 { //(u8, ([bool; 4], [bool; 4], [bool; 4], [bool; 4], [bool; 4], [bool; 4], [bool; 4], [bool; 4], [bool; 4], [bool; 4], [bool; 4], [bool; 4], [bool; 4], [bool; 4], [bool; 4], [bool; 4], [bool; 4], [bool; 4], [bool; 4], [bool; 4], [bool; 4], [bool; 4], [bool; 4], [bool; 4]), [bool; 12], (usize, usize, usize, usize, usize, usize, usize, usize, usize, usize, usize, usize) , ([bool;4],[bool;4]) ) {
          
    // u8 solving id, currently steppedformula
        
    
    let mut f = generateFormula(a,b);
    let nonsenseassignment = ([false;4], [false;4]);
    
    let dummystorage = StorageType {
        f: ( nonsenseassignment , f ),
        specialflag: IsDummyStorage::RealValue
    };
    
  
    let mut ss = StackState {
      index: 0usize,
      arrayacc: [dummystorage; 255],
    };
    
  
    // ----------- CONTENT-WISE DEBUGGING --------------
    /*
    let v = first_giant_step(f,ss);
    let assignment = v.0;
    let tempdummy = next_giant_step(f, ss, assignment);
  
    let hasun = unitsearch(f);
  
    //let phi_prop = propagate(assignment,f);
    let phi_prop = propagate( ([true,false,false,false], [false,false,false,false]) ,f);
    f = phi_prop;
  
    let returneddebug = ((f.clauses[0].matP, f.clauses[0].matN, f.clauses[1].matP, f.clauses[1].matN, f.clauses[2].matP, f.clauses[2].matN, f.clauses[3].matP, f.clauses[3].matN, 
      f.clauses[4].matP, f.clauses[4].matN, f.clauses[5].matP, f.clauses[5].matN, f.clauses[6].matP, f.clauses[6].matN, f.clauses[7].matP, f.clauses[7].matN) , 
      (f.clauses[0].Nl, f.clauses[1].Nl, f.clauses[2].Nl, f.clauses[3].Nl, f.clauses[4].Nl, f.clauses[5].Nl, f.clauses[6].Nl, f.clauses[7].Nl )
      );
  
    //returneddebug
  
      
    let litlit = decision(phi_prop );
    litlit
    */
  
    //(hasun.avalue, hasun.bvalue)
  
    
  
    // ---------------------------------------- ACTUAL CODE -------------------------------------
    
    let mut v = first_giant_step(f,ss);
    // v = (assignment, ObliviousStack) 
    let mut a = v.0;
    let mut os_o = v.1;
    let mut result = 0u8;
    let mut satresult = Sol::Unknown;
    
    for i in [0usize,0usize,0usize,0usize,0usize,0usize,0usize,0usize,0usize,0usize] {
      
      if satresult == Sol::Unknown {
        let v = next_giant_step(f, os_o, a);   // -> (Sol, assignment, StackState, Formula)
        a = v.1;
        os_o = v.2;
        f = v.3;
        satresult = v.0;
      }
      
  
      if satresult == Sol::Sat {
        result = 2u8;
      }
      if satresult == Sol::Unsat {
        result = 1u8;
      }
      
  
    }



    //let v = unitsearch(f);
    //let b_unit = v.bvalue;
    //let a_unit = v.avalue;
  
    let returneddebug = (result ,  (f.clauses[0].matP, f.clauses[0].matN, 
                                    f.clauses[1].matP, f.clauses[1].matN, 
                                    f.clauses[2].matP, f.clauses[2].matN, 
                                    f.clauses[3].matP, f.clauses[3].matN, 
                                    f.clauses[4].matP, f.clauses[4].matN, 
                                    f.clauses[5].matP, f.clauses[5].matN, 
                                    f.clauses[6].matP, f.clauses[6].matN, 
                                    f.clauses[7].matP, f.clauses[7].matN,
                                    f.clauses[8].matP, f.clauses[8].matN,
                                    f.clauses[9].matP, f.clauses[9].matN,
                                    f.clauses[10].matP, f.clauses[10].matN,
                                    f.clauses[11].matP, f.clauses[11].matN,
                                  ) , 
                                    f.alive,
                                   (f.clauses[0].Nl, f.clauses[1].Nl, 
                                    f.clauses[2].Nl, f.clauses[3].Nl, 
                                    f.clauses[4].Nl, f.clauses[5].Nl, 
                                    f.clauses[6].Nl, f.clauses[7].Nl,
                                    f.clauses[8].Nl, f.clauses[9].Nl,
                                    f.clauses[10].Nl, f.clauses[11].Nl ),
      a
        );

        result
    
    /*
    let sigma = check(a, f);
    let mut satresult =  Sol::Unknown;
    
    if sigma == CheckCode::Zero {
      satresult = Sol::Sat;
    }
    
    let b_conflict = sigma == CheckCode::One;
    
    //returneddebug

    //b_conflict


    let nonsenseassignment = ([true;4], [true;4]);
    let dummyvariableemptystack = StorageType {
      f: (nonsenseassignment, f),
      specialflag: IsDummyStorage::Dummy
    };
    
    let dummyvariablecondfalse = StorageType {
      f: (nonsenseassignment, f),
      specialflag: IsDummyStorage::DummyPrime
    };
    
    //condpop( cond: bool, stackstate: StackState, dummyvariableemptystack: StorageType, dummyvariablecondfalse: StorageType) -> (StorageType,StackState) {
    
    let res1 = condpop(b_conflict, os_o, dummyvariableemptystack, dummyvariablecondfalse); 
    //os_o = res1.1;
    let outvar = res1.0;
    let returntype = outvar.specialflag;
    if (returntype == IsDummyStorage::Dummy) == true {  // testing if dummyvariableemptystack
      satresult = Sol::Unsat; 
    }
    let arrayacc = os_o.arrayacc;
    let v = arrayacc[1usize].f;
    let p = v.0;
    returntype

     */

    
  //enum IsDummyStorage { 
  //  RealValue,
  //  Dummy,
  //  DummyPrime
  //}

  //struct StorageType {
  //  f: ( ([bool;4],[bool;4]) , Formula ),
  //  specialflag: IsDummyStorage
  
    

    /* 
    let allassignments = generateEmptyClause();
    let mut allassignmentsP = allassignments.matP;
    let mut allassignmentsN = allassignments.matN;
    
    let arrayacc = os_o.arrayacc;
    for countercounter in 0usize..254usize {
      let v = arrayacc[countercounter];
      let v1 = v.f;
      let a = v1.0;
      
      let amatp = a.0;
      let amatn = a.1;
      
      for idx in [0usize, 1usize, 2usize, 3usize] { 
        
        allassignmentsP[idx] = amatp[idx] | allassignmentsP[idx];
        allassignmentsN[idx] = amatn[idx] | allassignmentsN[idx];

      }

    }



    let asdf1 = ([true, true,false,false],[false,false,true,true]);
    let asdf2 = f;
    let valuetopush = StorageType {
      f: (asdf1, asdf2),
      specialflag: IsDummyStorage::RealValue
    };

    os_o = condpush( true , valuetopush, os_o );
    let asdf1 = ([true, false,true,false],[true,true,true,true]);
    let asdf2 = f;
    let valuetopush = StorageType {
      f: (asdf1, asdf2),
      specialflag: IsDummyStorage::RealValue
    };

    os_o = condpush( true , valuetopush, os_o );
    os_o = condpush( true , valuetopush, os_o );
    let arrayacc = os_o.arrayacc;

    let v = arrayacc[1usize].f;
    let p = v.0;

    p
    //(allassignmentsP,allassignmentsN)

    //a_unit
    
    */
  
  
    
  
    //  -------------------------------------- TEST CODE ------------------------------------------------- 
  
    //let iaunit = invertliteral(aunit); 
  
    //let reducedclause = removeliteralfromclause(iaunit , sampleclause );
  
    //let newformula = removeclausefromformula(f,  1usize);
  
    //let newf = propagate(aunit , f); 
    //if bunit == true {
    //f = propagate( aunit , f);
    //  f = generateFormula(a,b);
    //}
    //let doescontaina = clausecontainsl(aunit , sampleclause);
    //let v = reducedclause.matP;
  
    //let result = phiisempty(f);
    
    /*let returncheckcode = check(([true,false,false,false] , [false,false,false,false]) , f); //   check(l: ( [bool;4] , [bool;4] )  , f: Formula) ->  CheckCode
    let mut result = 0u8;
    
    if returncheckcode == CheckCode::Zero {
      result = 0u8;
    }
    if returncheckcode == CheckCode::One {
      result = 1u8;
    }
    if returncheckcode == CheckCode::Two {
      result = 2u8;
    }*/
    
    /*
    let asdf = first_giant_step(f, ss);  //first_giant_step(f: Formula, os: ObliviousStack) -> (assignment, ObliviousStack) 
    ss = asdf.1;
    let newa = asdf.0;
  
    // next_giant_step(f: Formula, os:StackState, a: ([bool; 4], [bool; 4])) -> (Sol, bool, StackState, Formula) 
    let ngs = next_giant_step(f, ss, newa);  //-> (Sol, bool, ObliviousStack, Formula)
  
    let ppp = asdf.0;
  
    result
    
    */
  
  
  }
  
  /*
  
  
  variables: 1,2,3,4
  
  1:true  2:true  3:false   4:false
  
  
  A: (1 , 2)  ,  (-1, -3)  , (-2,  -4) , (-3, -4)
  B: (2,  3)  ,  (2,   4)  , (2 ,   -4), (1)
  
  A: ( [([1,1,0,0],[0,0,0,0]) , ([0,0,0,0],[1,0,1,0])  , ([0,0,0,0],[0,1,0,1]) , ([0,0,0,0],[0,0,1,1])] , [2usize, 2usize, 2usize, 2usize])
  ( [([true,true,false,false],[false,false,false,false]) , ([false,false,false,false],[true,false,true,false])  , ([false,false,false,false],[false,true,false,true]) , ([false,false,false,false],[false,false,true,true])] , [2usize, 2usize, 2usize, 2usize])
  
  
  B: ( [([false,true,true,false],[false,false,false,false]), ([false,true,false,true],[false,false,false,false]) , ([false,true,false,false],[false,false,false,true]) , ([true,false,false,false],[false,false,false,false])] , [2usize, 2usize, 2usize, 1usize])
  
  
  [ ([bool;4],[bool;4]),([bool;4],[bool;4]),([bool;4],[bool;4]),([bool;4],[bool;4])    , ([bool;4],[bool;4]),([bool;4],[bool;4]),([bool;4],[bool;4]),([bool;4],[bool;4])] , [usize; 8]
  




    (1 , 2)  ,  (-1, -3)  , (-2,  -4) , (-3, -4),    (-1 -2 3 -4)  (-2 -3 4) (2,  3)  ,  (2,   4)  , (2 ,   -4), (-1 -2 -3),  (-2 3 -4)     (-1 2 3 4)
    4:true
    (1 , 2)  ,  (-1, -3)  , (-2) , (-3),    (-1 -2 3)  (2,  3)  ,   , (2 ), (-1 -2 -3),  (-2 3) 
    2:true
     ,  (-1, -3)  , () , (-3),    (-1 3)    ,   , , (-1 -3),  ( 3) 



    1:true  2:true  3:false   4:false
    
    
    A: (1 , 2)  ,  (-1, -3)  , (-2,  -4) , (-3, -4),    (-1 -2 3 -4)  (-2 -3 4)      #6

    
    A: ( [ ([true,true,false,false],[false,false,false,false]) ,
           ([false,false,false,false],[true,false,true,false]) , 
           ([false,false,false,false],[false,true,false,true]) ,
           ([false,false,false,false],[false,false,true,true]) ,
           ([false,false,true,false] ,[true,true,false,true])  ,
           ([false,false,false,true],[false,true,true,false])  ]     , [2usize,2usize,2usize,2usize,4usize,3usize])


    B: (2,  3)  ,  (2,   4)  , (2 ,   -4), (-1 -2 -3),  (-2 3 -4)     (-1 2 3 4)      #6
    B  ( [ ([false,true,true,false], [false,false,false,false]), 
           ([false,true,false,true],[false,false,false,false]), 
           ([false,true,false,false],[false,false,false,true]), 
           ([false,false,false,false], [true,true,true,false]), 
           ([false,false,true,false],[false,true,false,true]), 
           ([false,true,true,true],  [true,false,false,false])  ]     , [2usize,2usize,2usize,3usize,3usize,4usize])






  */  
  