import { Button, Image, ScrollView, Text, View } from "react-native";
import { fontSize } from "../constants/style";

function Home({navigation}: {navigation: any}) {
    return <ScrollView contentInsetAdjustmentBehavior='automatic' style={{
        backgroundColor: 'black',
        minHeight: '100%',
        padding: 20,
        paddingTop: 20,
        }}>
            <View style={{
                display: 'flex',
                justifyContent: 'flex-start',
                alignItems: 'center',
                flexDirection: 'row',
            }}>
                <Image source={require('../assets/profile.png')} style={{
                    width: fontSize.medium,
                    aspectRatio: 1,

                    objectFit: 'contain',
                    marginRight: 15,
                }}/>
                <Text style={{
                    fontSize: fontSize.small,
                    color: 'white',
                    flex:1
                }}>
                    this is home huh
                </Text>
            </View>
            <Button title="to login" onPress={() => {
                console.log("to login");
                navigation.navigate('login');
            }}/>
    </ScrollView>
}

export default Home;
