import { Image, Pressable, SafeAreaView, ScrollView, Text, View } from "react-native";
import Navbar from "../components/Navbar";
import { DefaultContainer, colorScheme, defaultFont, defaultFontBold, fontSize } from "../constants/style";
import { Fragment } from "react";
import { PercentageGrowthIndicator, StockData } from "../components/StockComponents";

function Inspect({navigation, route}: {navigation: any, route: any}) {
    let stockID = route.params.stockID;
    // let stockData = new StockData(stockID, name:)
    let stockData = new StockData(stockID, '', '', 0, 1.32);

    return <DefaultContainer menu="inspect">
        <View style={{
            display:'flex',
            justifyContent:'center',
            alignItems:'center',
            flexDirection:'row',
            // backgroundColor:'purple'
        }}>
            <Pressable onPress={() => {navigation.goBack(null)}} style={{
                display:'flex',
                justifyContent:'center',
                alignItems:'center',
                height:'100%',
                position:'absolute',
                left:10,
                // backgroundColor:'red'
            }}>
                <Image source={require('../assets/arrow_left.png')} style={{
                    height:'60%',
                    aspectRatio:2,
                    objectFit:'contain',
                    // backgroundColor:'green'
                }} />
            </Pressable>
            <View style={{
                display:'flex',
                justifyContent:'center',
                alignItems:'center',
                flexDirection:'row'
            }}>
                <Text style={{
                    fontFamily:defaultFontBold,
                    color:colorScheme.primary,
                    fontSize:fontSize.medium
                }}>
                    nflx
                </Text>
                <PercentageGrowthIndicator growthAmount={stockData.growthAmount}/>
            </View>
        </View>
        <ScrollView style={{
            backgroundColor:colorScheme.background,
            flex: 1,
            padding: 20,
            paddingTop: 20,
        }} contentInsetAdjustmentBehavior='automatic'>
            <Text style={{
                fontFamily:defaultFont,
                fontSize: fontSize.small,
                color:colorScheme.primary,
            }}>
                stock with id {`${stockID}`}
            </Text>
        </ScrollView>
    </DefaultContainer>

    // return <Fragment>
    //     <SafeAreaView style={{
    //         flex:0,
    //         backgroundColor:colorScheme.background
    //     }}/>
    //     <SafeAreaView style={{
    //         flex:1,
    //         backgroundColor:colorScheme.secondary
    //     }}>
    //         <View style={{
    //             flex:1,
    //             backgroundColor:'purple'
    //         }}>

    //         </View>
    //         <Navbar menu="home" />
    //     </SafeAreaView>
    // </Fragment>

    // return <View style={{
    //     display: 'flex',
    //     flexDirection: 'column',
    //     height: '100%',
    // }}>
    //     <ScrollView style={{
    //         backgroundColor:colorScheme.background,
    //         flex: 1,
    //         padding: 20,
    //         paddingTop: 10,
    //     }} contentInsetAdjustmentBehavior='automatic'>
    //         <Text style={{
    //             fontFamily:defaultFont,
    //             fontSize: fontSize.small,
    //             color:colorScheme.primary,
    //         }}>
    //             stock with id {`${stockID}`}
    //         </Text>
    //     </ScrollView>
    //     <Navbar menu="inspect"/>
    // </View>

    // return <View style={{
    //     display: 'flex',
    //     flexDirection: 'column',
    //     height: '100%',
    //     // marginTop:50
    //     backgroundColor:colorScheme.background
    // }}
    // >
    //     {/* <View style={{
    //         backgroundColor: 'red'
    //     }}>
    //         <Pressable onPress={() => {console.log("back button is pressed");}}>
    //             <Image source={require('../assets/arrow_left.png')} style={{
    //                 height:20,
    //                 aspectRatio:2,
    //                 // width:'auto',
    //                 objectFit:'contain',
    //                 backgroundColor:'green'
    //             }} />
    //         </Pressable>
    //     </View> */}
    //     <ScrollView style={{
    //         backgroundColor:colorScheme.background,
    //         flex: 1,
    //         padding: 20,
    //         paddingTop: 20,
    //     }} contentInsetAdjustmentBehavior='automatic'>
    //         <Text style={{
    //             fontFamily:defaultFont,
    //             fontSize: fontSize.small,
    //             color:colorScheme.primary,
    //         }}>
    //             stock with id {`${stockID}`}
    //         </Text>
    //     </ScrollView>
    //     <Navbar menu="inspect"/>
    // </View>;
}

export default Inspect;
