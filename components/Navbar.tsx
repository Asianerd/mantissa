import { Image, Pressable, StyleSheet, View } from "react-native";
import { colorScheme } from "../constants/style";

const navbarStyles = StyleSheet.create({
    icon: {
        width:27,
        aspectRatio:1,
        objectFit: 'contain'
    }
})

function Navbar({menu}: {menu: String}) {
    function NavbarButton(name: String, iconName: String, active: boolean) {
        return (
            <Pressable style={navbarStyles.icon} onPress={() => { console.log(`${name} tab`); }}>
                <Image source={require(`../assets/${iconName}.png`)} style={{ height:'100%', width:'100%', opacity:active ? 1.0 : 0.5 }}/>
            </Pressable>
        );
    }

    return (
        <View style={{
            backgroundColor:colorScheme.secondary,
            height: 50,

            display:'flex',
            justifyContent:'space-evenly',
            alignItems:'center',
            flexDirection:'row'
        }}>
            <Pressable style={navbarStyles.icon} onPress={() => { console.log("search tab"); }}>
                <Image source={require('../assets/search.png')} style={{ height:'100%', width:'100%', opacity:0.5 }}/>
            </Pressable>
            <Pressable style={navbarStyles.icon} onPress={() => { console.log("wallet tab"); }}>
                <Image source={require('../assets/finance.png')} style={{ height:'100%', width:'100%', opacity:0.5 }}/>
            </Pressable>
            <Pressable style={navbarStyles.icon} onPress={() => { console.log("profile tab"); }}>
                <Image source={require('../assets/profile.png')} style={{ height:'100%', width:'100%', opacity:0.5 }}/>
            </Pressable>
        </View>
    );
}

export default Navbar;