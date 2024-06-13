// SPDX-License-Identifier: GPL-3.0
/*
    Copyright 2021 0KIMS association.

    This file is generated with [snarkJS](https://github.com/iden3/snarkjs).

    snarkJS is a free software: you can redistribute it and/or modify it
    under the terms of the GNU General Public License as published by
    the Free Software Foundation, either version 3 of the License, or
    (at your option) any later version.

    snarkJS is distributed in the hope that it will be useful, but WITHOUT
    ANY WARRANTY; without even the implied warranty of MERCHANTABILITY
    or FITNESS FOR A PARTICULAR PURPOSE. See the GNU General Public
    License for more details.

    You should have received a copy of the GNU General Public License
    along with snarkJS. If not, see <https://www.gnu.org/licenses/>.
*/

pragma solidity >=0.7.0 <0.9.0;

contract Groth16Verifier {
    // Scalar field size
    uint256 constant r    = 21888242871839275222246405745257275088548364400416034343698204186575808495617;
    // Base field size
    uint256 constant q   = 21888242871839275222246405745257275088696311157297823662689037894645226208583;

    // Verification Key data
    uint256 constant alphax  = 20491192805390485299153009773594534940189261866228447918068658471970481763042;
    uint256 constant alphay  = 9383485363053290200918347156157836566562967994039712273449902621266178545958;
    uint256 constant betax1  = 4252822878758300859123897981450591353533073413197771768651442665752259397132;
    uint256 constant betax2  = 6375614351688725206403948262868962793625744043794305715222011528459656738731;
    uint256 constant betay1  = 21847035105528745403288232691147584728191162732299865338377159692350059136679;
    uint256 constant betay2  = 10505242626370262277552901082094356697409835680220590971873171140371331206856;
    uint256 constant gammax1 = 11559732032986387107991004021392285783925812861821192530917403151452391805634;
    uint256 constant gammax2 = 10857046999023057135944570762232829481370756359578518086990519993285655852781;
    uint256 constant gammay1 = 4082367875863433681332203403145435568316851327593401208105741076214120093531;
    uint256 constant gammay2 = 8495653923123431417604973247489272438418190587263600148770280649306958101930;
    uint256 constant deltax1 = 10878135220536454536947380801232800677011556167007128458228442619582416572224;
    uint256 constant deltax2 = 472933731001167927440038191064549326908050635293100536809069492613570662499;
    uint256 constant deltay1 = 10308226552679841484235563729305337754130452194775404286523194444548588689331;
    uint256 constant deltay2 = 5997486235209158902030151543344992031536495559721543443856766798014304480631;

    
    uint256 constant IC0x = 10662492772067662198637701174188715040707263197279968717514501833693985498120;
    uint256 constant IC0y = 5048878444811739574307673200319887322456246304144079411779870398787481709060;
    
    uint256 constant IC1x = 10631623844979933886430037559067789969834034554638779030068313227902759600609;
    uint256 constant IC1y = 20244193372465334172210866712147736699506363662343455250170678558616492152513;
    
    uint256 constant IC2x = 880132320576327775901304244778419484522016536254814102534629024483734680632;
    uint256 constant IC2y = 14994207343550551307870202296499337090459448005334128322007799218211060083407;
    
    uint256 constant IC3x = 13419227474750031546185708197069145250771275606764466289858762659547194626699;
    uint256 constant IC3y = 19399288206247604934599686659152450664445742801675811989687507709851619960684;
    
    uint256 constant IC4x = 8581236415557518929661024567632590934279356037654754017487481114796723384106;
    uint256 constant IC4y = 5000921707034135501025489124307319634332429053890101925948898409803956298703;
    
    uint256 constant IC5x = 2647142864391893725112513160394235653438736968871275695456269007636622585855;
    uint256 constant IC5y = 20130586282117915164957178691106968032130270993689994029458229850912667543804;
    
    uint256 constant IC6x = 5689897199615581937292615472205900550038094783868723341434136832530650162807;
    uint256 constant IC6y = 10278997303367685758264083316217005381178786823157620631859496309138156454197;
    
    uint256 constant IC7x = 20806607515578934788612662558669434854242443146621354343710733269259097612138;
    uint256 constant IC7y = 8756385226631641726143195547861786371780298110839147079271467726302155054094;
    
    uint256 constant IC8x = 10930592961557633713436109701646865952385495742911411646414103690515754678076;
    uint256 constant IC8y = 775180359052150904868505681004582738845543076098839462514179757731042681760;
    
    uint256 constant IC9x = 10499757337410698685538930635490827169075403595686497938301007455066676537577;
    uint256 constant IC9y = 2467626203240578478563322262576368763925535138503778791789786664152196993493;
    
    uint256 constant IC10x = 11406304298253244138758937330418548508778092246909600152214177025819858947144;
    uint256 constant IC10y = 2504697483902767590782432222264652816415263674983250800541715306192378880010;
    
    uint256 constant IC11x = 5604605783786227786414902726795782453366542844133566801376280324012569538271;
    uint256 constant IC11y = 1175833659496395765547225908366231868681752338619764047966552849304486770165;
    
    uint256 constant IC12x = 13974499340762612806425255021031519138303935176166210963126351154496373983084;
    uint256 constant IC12y = 10018279763669727837318713639269427706220009762775265540800436607105366707624;
    
    uint256 constant IC13x = 10907846539315397590240961146106370933256556934704498944866083238000649814274;
    uint256 constant IC13y = 13177161512123423435824460326284653347929568963669840077036952699646825127350;
    
    uint256 constant IC14x = 11389329712393033255379358847380379668278803176242064751267123446409203706725;
    uint256 constant IC14y = 650998383004409178742339958548323682031937961883742441602530067550006769038;
    
    uint256 constant IC15x = 18900390739697548583839800879679190573396586579216377828001186357575884090283;
    uint256 constant IC15y = 15991859854193878552861962225261393786825182190151345521605624663832926178814;
    
    uint256 constant IC16x = 18688003573333024523529661104475353763309901380193414438097775444809038650882;
    uint256 constant IC16y = 14419367193690598311809224961381803874301240943831396922124317247590300667157;
    
    uint256 constant IC17x = 17032406485548752930729277472464345865671771261403134483233536025540868761344;
    uint256 constant IC17y = 11015026262259791837683783384375546710883623636957982592193251022172409551869;
    
    uint256 constant IC18x = 6704219536438594923593173087813077136500430578994054214398194446754053303343;
    uint256 constant IC18y = 2402239745339009650084330082678399990175784483062783991240681608636090065910;
    
    uint256 constant IC19x = 2584967349701968404930120965422501912970981569066371835873341310137005009041;
    uint256 constant IC19y = 16453496223902817677715314093186258958112549446581083926326652584973044184014;
    
    uint256 constant IC20x = 5656890661734115447711468233534406898374820422717765031463444766244099150245;
    uint256 constant IC20y = 19497924949154560945230970240350093684936842413878609575488105960326249495085;
    
    uint256 constant IC21x = 3305793582321172216035320100489654491737225805410378226725610932752199922750;
    uint256 constant IC21y = 9135695930705280491053728682322285626931500525084567278689635104691830613988;
    
    uint256 constant IC22x = 10634500258176165063788505032709508778019816951124183370814562915953989194819;
    uint256 constant IC22y = 20743715763711076029713719912993564753447949640967563162484139820339863278609;
    
    uint256 constant IC23x = 15803938473075813588145490211060098389335308793413126918943808341886300231917;
    uint256 constant IC23y = 12777585743719080289983093712911579369583322671706441003375949088999091266073;
    
    uint256 constant IC24x = 21137293935252541322372530492233710439005064385228710439265992564039189533085;
    uint256 constant IC24y = 12105792454184642914943334339843247688572987258792852469053137805345342592159;
    
    uint256 constant IC25x = 6311896794463120719391631641464174093163844856255640975778962104024965657146;
    uint256 constant IC25y = 13863883946840407948504522942347862007475758904125339525878539757485082995280;
    
    uint256 constant IC26x = 6760116149067421849735433825636399698019491145884663607538459860913202080431;
    uint256 constant IC26y = 17905090709798595116668145027276672543567312657260849635834918635300424803745;
    
    uint256 constant IC27x = 127709658826332752759774893434672426669213605575876222994935552868257007258;
    uint256 constant IC27y = 5889699472392658240737054505709168581330251198201495528724651949336117038417;
    
    uint256 constant IC28x = 13847132774163277148418420732516327818386709882543725698449403114474122582495;
    uint256 constant IC28y = 16544449405403483779270415587326511155747620783971382704725281647789118279667;
    
    uint256 constant IC29x = 7249363045196280468108511322244198319976500006996186898615215592650470067905;
    uint256 constant IC29y = 5433985042780853091627386295983492304441633648726130912171862020154029431873;
    
    uint256 constant IC30x = 21870636207333155100745035605929765656827935014151922240395102832936549966863;
    uint256 constant IC30y = 6149278343184439954385284461680440519024593753403104862864036151763317224080;
    
    uint256 constant IC31x = 12516415477396031365650982071956057386821071618847460925398600646719465118423;
    uint256 constant IC31y = 21662190606541609916933558378116913431392810940835840753920677404418729497133;
    
 
    // Memory data
    uint16 constant pVk = 0;
    uint16 constant pPairing = 128;

    uint16 constant pLastMem = 896;

    function verifyProof(uint[2] calldata _pA, uint[2][2] calldata _pB, uint[2] calldata _pC, uint[31] calldata _pubSignals) public view returns (bool) {
        assembly {
            function checkField(v) {
                if iszero(lt(v, q)) {
                    mstore(0, 0)
                    return(0, 0x20)
                }
            }
            
            // G1 function to multiply a G1 value(x,y) to value in an address
            function g1_mulAccC(pR, x, y, s) {
                let success
                let mIn := mload(0x40)
                mstore(mIn, x)
                mstore(add(mIn, 32), y)
                mstore(add(mIn, 64), s)

                success := staticcall(sub(gas(), 2000), 7, mIn, 96, mIn, 64)

                if iszero(success) {
                    mstore(0, 0)
                    return(0, 0x20)
                }

                mstore(add(mIn, 64), mload(pR))
                mstore(add(mIn, 96), mload(add(pR, 32)))

                success := staticcall(sub(gas(), 2000), 6, mIn, 128, pR, 64)

                if iszero(success) {
                    mstore(0, 0)
                    return(0, 0x20)
                }
            }

            function checkPairing(pA, pB, pC, pubSignals, pMem) -> isOk {
                let _pPairing := add(pMem, pPairing)
                let _pVk := add(pMem, pVk)

                mstore(_pVk, IC0x)
                mstore(add(_pVk, 32), IC0y)

                // Compute the linear combination vk_x
                
                g1_mulAccC(_pVk, IC1x, IC1y, calldataload(add(pubSignals, 0)))
                
                g1_mulAccC(_pVk, IC2x, IC2y, calldataload(add(pubSignals, 32)))
                
                g1_mulAccC(_pVk, IC3x, IC3y, calldataload(add(pubSignals, 64)))
                
                g1_mulAccC(_pVk, IC4x, IC4y, calldataload(add(pubSignals, 96)))
                
                g1_mulAccC(_pVk, IC5x, IC5y, calldataload(add(pubSignals, 128)))
                
                g1_mulAccC(_pVk, IC6x, IC6y, calldataload(add(pubSignals, 160)))
                
                g1_mulAccC(_pVk, IC7x, IC7y, calldataload(add(pubSignals, 192)))
                
                g1_mulAccC(_pVk, IC8x, IC8y, calldataload(add(pubSignals, 224)))
                
                g1_mulAccC(_pVk, IC9x, IC9y, calldataload(add(pubSignals, 256)))
                
                g1_mulAccC(_pVk, IC10x, IC10y, calldataload(add(pubSignals, 288)))
                
                g1_mulAccC(_pVk, IC11x, IC11y, calldataload(add(pubSignals, 320)))
                
                g1_mulAccC(_pVk, IC12x, IC12y, calldataload(add(pubSignals, 352)))
                
                g1_mulAccC(_pVk, IC13x, IC13y, calldataload(add(pubSignals, 384)))
                
                g1_mulAccC(_pVk, IC14x, IC14y, calldataload(add(pubSignals, 416)))
                
                g1_mulAccC(_pVk, IC15x, IC15y, calldataload(add(pubSignals, 448)))
                
                g1_mulAccC(_pVk, IC16x, IC16y, calldataload(add(pubSignals, 480)))
                
                g1_mulAccC(_pVk, IC17x, IC17y, calldataload(add(pubSignals, 512)))
                
                g1_mulAccC(_pVk, IC18x, IC18y, calldataload(add(pubSignals, 544)))
                
                g1_mulAccC(_pVk, IC19x, IC19y, calldataload(add(pubSignals, 576)))
                
                g1_mulAccC(_pVk, IC20x, IC20y, calldataload(add(pubSignals, 608)))
                
                g1_mulAccC(_pVk, IC21x, IC21y, calldataload(add(pubSignals, 640)))
                
                g1_mulAccC(_pVk, IC22x, IC22y, calldataload(add(pubSignals, 672)))
                
                g1_mulAccC(_pVk, IC23x, IC23y, calldataload(add(pubSignals, 704)))
                
                g1_mulAccC(_pVk, IC24x, IC24y, calldataload(add(pubSignals, 736)))
                
                g1_mulAccC(_pVk, IC25x, IC25y, calldataload(add(pubSignals, 768)))
                
                g1_mulAccC(_pVk, IC26x, IC26y, calldataload(add(pubSignals, 800)))
                
                g1_mulAccC(_pVk, IC27x, IC27y, calldataload(add(pubSignals, 832)))
                
                g1_mulAccC(_pVk, IC28x, IC28y, calldataload(add(pubSignals, 864)))
                
                g1_mulAccC(_pVk, IC29x, IC29y, calldataload(add(pubSignals, 896)))
                
                g1_mulAccC(_pVk, IC30x, IC30y, calldataload(add(pubSignals, 928)))
                
                g1_mulAccC(_pVk, IC31x, IC31y, calldataload(add(pubSignals, 960)))
                

                // -A
                mstore(_pPairing, calldataload(pA))
                mstore(add(_pPairing, 32), mod(sub(q, calldataload(add(pA, 32))), q))

                // B
                mstore(add(_pPairing, 64), calldataload(pB))
                mstore(add(_pPairing, 96), calldataload(add(pB, 32)))
                mstore(add(_pPairing, 128), calldataload(add(pB, 64)))
                mstore(add(_pPairing, 160), calldataload(add(pB, 96)))

                // alpha1
                mstore(add(_pPairing, 192), alphax)
                mstore(add(_pPairing, 224), alphay)

                // beta2
                mstore(add(_pPairing, 256), betax1)
                mstore(add(_pPairing, 288), betax2)
                mstore(add(_pPairing, 320), betay1)
                mstore(add(_pPairing, 352), betay2)

                // vk_x
                mstore(add(_pPairing, 384), mload(add(pMem, pVk)))
                mstore(add(_pPairing, 416), mload(add(pMem, add(pVk, 32))))


                // gamma2
                mstore(add(_pPairing, 448), gammax1)
                mstore(add(_pPairing, 480), gammax2)
                mstore(add(_pPairing, 512), gammay1)
                mstore(add(_pPairing, 544), gammay2)

                // C
                mstore(add(_pPairing, 576), calldataload(pC))
                mstore(add(_pPairing, 608), calldataload(add(pC, 32)))

                // delta2
                mstore(add(_pPairing, 640), deltax1)
                mstore(add(_pPairing, 672), deltax2)
                mstore(add(_pPairing, 704), deltay1)
                mstore(add(_pPairing, 736), deltay2)


                let success := staticcall(sub(gas(), 2000), 8, _pPairing, 768, _pPairing, 0x20)

                isOk := and(success, mload(_pPairing))
            }

            let pMem := mload(0x40)
            mstore(0x40, add(pMem, pLastMem))

            // Validate that all evaluations ∈ F
            
            checkField(calldataload(add(_pubSignals, 0)))
            
            checkField(calldataload(add(_pubSignals, 32)))
            
            checkField(calldataload(add(_pubSignals, 64)))
            
            checkField(calldataload(add(_pubSignals, 96)))
            
            checkField(calldataload(add(_pubSignals, 128)))
            
            checkField(calldataload(add(_pubSignals, 160)))
            
            checkField(calldataload(add(_pubSignals, 192)))
            
            checkField(calldataload(add(_pubSignals, 224)))
            
            checkField(calldataload(add(_pubSignals, 256)))
            
            checkField(calldataload(add(_pubSignals, 288)))
            
            checkField(calldataload(add(_pubSignals, 320)))
            
            checkField(calldataload(add(_pubSignals, 352)))
            
            checkField(calldataload(add(_pubSignals, 384)))
            
            checkField(calldataload(add(_pubSignals, 416)))
            
            checkField(calldataload(add(_pubSignals, 448)))
            
            checkField(calldataload(add(_pubSignals, 480)))
            
            checkField(calldataload(add(_pubSignals, 512)))
            
            checkField(calldataload(add(_pubSignals, 544)))
            
            checkField(calldataload(add(_pubSignals, 576)))
            
            checkField(calldataload(add(_pubSignals, 608)))
            
            checkField(calldataload(add(_pubSignals, 640)))
            
            checkField(calldataload(add(_pubSignals, 672)))
            
            checkField(calldataload(add(_pubSignals, 704)))
            
            checkField(calldataload(add(_pubSignals, 736)))
            
            checkField(calldataload(add(_pubSignals, 768)))
            
            checkField(calldataload(add(_pubSignals, 800)))
            
            checkField(calldataload(add(_pubSignals, 832)))
            
            checkField(calldataload(add(_pubSignals, 864)))
            
            checkField(calldataload(add(_pubSignals, 896)))
            
            checkField(calldataload(add(_pubSignals, 928)))
            
            checkField(calldataload(add(_pubSignals, 960)))
            
            checkField(calldataload(add(_pubSignals, 992)))
            

            // Validate all evaluations
            let isValid := checkPairing(_pA, _pB, _pC, _pubSignals, pMem)

            mstore(0, isValid)
             return(0, 0x20)
         }
     }
 }
