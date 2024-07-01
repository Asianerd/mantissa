import { Button, Image, Pressable, ScrollView, StyleSheet, Text, View } from "react-native";
import { colorScheme, defaultFont, defaultFontBold, defaultFontItalic, fontSize } from "../constants/style";
import { screenSize } from "../App";
import Navbar from "../components/Navbar";
import StockWidget, { HyperlinkWidget, StockData } from "../components/StockComponents";

function Home({navigation, route}: {navigation: any, route: any}) {
    const username = route.params.username;
    const password = route.params.password;

    function HyperlinkHeader() {
        return (
            <View style={{
                marginBottom: 10
            }}>
                <Text style={{ fontFamily: defaultFontItalic, fontSize:fontSize.tiny * 1.3 }}>
                    fastest growing shares →
                </Text>
            </View>
        );
    }

    const stockData = new StockData('netflix.com', 'NFLX', 647.15, 0.2);

    return <View style={{
        display: 'flex',
        flexDirection: 'column',
        height: '100%'
    }}>
        <ScrollView contentInsetAdjustmentBehavior='automatic' style={{
            backgroundColor: colorScheme.background,
            flex: 1,
            padding: 20,
            paddingTop: 20,
        }}>
            <Text style={{
                fontFamily:defaultFont,
                fontSize: fontSize.small,
                color:colorScheme.primary,
            }} >
                good evening, {username}
            </Text>
            {/* <View style={{
                // backgroundColor:'red',
                height:fontSize.large,
                display: 'flex',
                justifyContent: 'space-between',
                alignItems: 'center',
                flexDirection: 'row-reverse',
            }}>
                <Image source={require('../assets/profile.png')} style={{ width:fontSize.large, objectFit:'contain' }} />
                <Text style={{
                    fontFamily:defaultFont,
                    fontSize: fontSize.small,
                    color:colorScheme.primary,
                    // marginLeft:15
                }} >
                    good evening, {username}
                </Text>
            </View> */}
            <View style={{
                marginTop: 20
            }}>
                <HyperlinkHeader />
                <ScrollView horizontal={true}>
                    <StockWidget stockData={stockData} />
                    <StockWidget stockData={stockData} style={{ marginLeft:20 }} />
                    <StockWidget stockData={stockData} style={{ marginLeft:20 }} />
                    <StockWidget stockData={stockData} style={{ marginLeft:20 }} />
                    <StockWidget stockData={stockData} style={{ marginLeft:20 }} />
                    <StockWidget stockData={stockData} style={{ marginLeft:20 }} />
                    <HyperlinkWidget func={() => { console.log("pressed"); }} style={{ marginLeft:20 }}/>
                </ScrollView>
            </View>
        </ScrollView>
        <Navbar menu="home"/>
    </View>
}

export default Home;
