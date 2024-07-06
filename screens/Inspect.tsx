import { ScrollView, Text, View } from "react-native";
import Navbar from "../components/Navbar";
import { colorScheme, defaultFont, fontSize } from "../constants/style";

function Inspect({navigation, route}: {navigation: any, route: any}) {
    let stockID = route.params.stockID;

    return <View
    style={{
        display: 'flex',
        flexDirection: 'column',
        height: '100%'
    }}
    >
        <ScrollView style={{
            backgroundColor:colorScheme.background,
            flex: 1,
            padding: 20,
            paddingTop: 20,
        }}>
            <Text style={{
                fontFamily:defaultFont,
                fontSize: fontSize.small,
                color:colorScheme.primary,
            }}>
                stock with id {`${stockID}`}
            </Text>
        </ScrollView>
        <Navbar menu="inspect"/>
    </View>;
}

export default Inspect;
