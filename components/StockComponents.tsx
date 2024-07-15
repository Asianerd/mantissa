import { Image, Pressable, Text, View } from "react-native";
import { colorScheme, defaultFont, defaultFontBold, defaultFontItalic, fontSize } from "../constants/style";
import { Circle, Polyline, Svg } from "react-native-svg";

const widgetStyles = {
    height: 160,
    width: 250,
    borderRadius: 20,
}

export class StockData {
    stockID: number;
    name: string;
    code: string;
    lastBuy: number;
    growthAmount: number;

    constructor(stockID: number, name: string, code: string, lastBuy: number, growthAmount: number) {
        this.stockID = stockID;
        this.name = name;
        this.code = code;
        this.lastBuy = lastBuy;
        this.growthAmount = growthAmount;
    }
}

function StockWidget({navigation, stockData, style}: {navigation: any, stockData: StockData, style?: any}) {
    const growthType = stockData.growthAmount > 0 ? 'profit' : stockData.growthAmount < 0 ? 'loss' : 'neutral';

    return (
        <Pressable style={[{
            backgroundColor:colorScheme.secondary,
            justifyContent:'flex-end',
            overflow: 'hidden'
        }, widgetStyles, style]}
        onPress={() => {
            navigation.navigate('inspect', { stockID: stockData.stockID })
        }}
        >
            <Svg style={{
                // backgroundColor:'red'
            }}>
                {/* <Circle cx="10" cy="10" r="45" fill="green" /> */}
                <Polyline points="0,150 30,50 60,100 60,100 90,110 120,90 150,100 180,123 210,53 240,100" fill="none" stroke="green" strokeWidth="1.5"/>
            </Svg>
            <View style={{
                position:'absolute',
                display:'flex',
                justifyContent:'space-between',
                alignItems:'center',
                flexDirection:'row',
                width:'100%',
                padding:10,
                paddingHorizontal:15
            }}>
                <View style={{
                    display:'flex',
                    flexDirection:'row',
                    alignItems: 'center'
                    // alignItems: 'flex-end'
                }}>
                    <Text style={{ fontFamily:defaultFontBold, fontSize:fontSize.large, color:colorScheme.primary }}>
                        {stockData.code}
                    </Text>
                    <View style={{
                        display:'flex',
                        flexDirection:'column',
                        alignItems:'center',
                        marginLeft: 10,
                        // marginBottom: 5
                    }}>
                        {
                            growthType == 'neutral' ? (
                                <Image source={require('../assets/neutral.png')} style={{
                                    height:15,
                                    aspectRatio:1
                                }}/>
                            ) : (
                                <>
                                    {
                                        growthType == 'loss' ? '' : (
                                            <Image source={require('../assets/profit.png')} style={{
                                                height:15,
                                                aspectRatio:1
                                            }}/>
                                        )
                                    }
                                    <Text style={{
                                        fontFamily:defaultFontBold,
                                        fontSize:fontSize.tiny,
                                        color: growthType == 'profit' ? '#0f0' : '#f00'
                                    }}>
                                        {stockData.growthAmount}
                                    </Text>
                                    {
                                        growthType == 'profit' ? '' : (
                                            <Image source={require('../assets/loss.png')} style={{
                                                height:15,
                                                aspectRatio:1
                                            }}/> 
                                        )
                                    }
                                </>
                            )
                        }
                    </View>
                </View>
                <Text style={{
                    fontFamily:defaultFontBold,
                    fontSize:fontSize.medium,
                    color:colorScheme.primary
                }}>
                    {stockData.lastBuy}
                </Text>
            </View>
        </Pressable>
    );
}

export function HyperlinkWidget({style, func}: { style?: any, func?: any }) {
    return (
        <Pressable onPress={func} style={[{
            backgroundColor:colorScheme.secondary,
            opacity:0.6,
            display:'flex',
            justifyContent:'center',
            alignItems:'center',
        }, widgetStyles, style ]}>
            <Text style={{
                color:colorScheme.primary,
                fontFamily:defaultFontItalic
            }}>
                see more
            </Text>
        </Pressable>
    );
}

export default StockWidget;