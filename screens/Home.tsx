import { Button, Image, Pressable, ScrollView, StyleSheet, Text, View } from "react-native";
import { colorScheme, defaultFont, fontSize } from "../constants/style";
import { screenSize } from "../App";
import Navbar from "../components/Navbar";

function Home({navigation, route}: {navigation: any, route: any}) {
    const username = route.params.username;
    const password = route.params.password;

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
            {/* <View style={{
                display: 'flex',
                justifyContent: 'flex-start',
                alignItems: 'center',
                flexDirection: 'row',
            }}>
                <Image source={require('../assets/profile.png')} style={{
                    width: fontSize.large,
                    aspectRatio: 1,

                    objectFit: 'contain',
                    marginRight: 15,
                }}/>
                <Text style={{
                    fontSize: fontSize.small,
                    fontFamily: defaultFont,
                    color: 'white',
                    flex:1
                }}>
                    {username}
                </Text>
            </View> */}
            <View>

            </View>
        </ScrollView>
        <Navbar menu="home"/>
    </View>
}

export default Home;
