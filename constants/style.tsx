import { ColorValue, StyleSheet, View } from "react-native"
import { Defs, LinearGradient, Rect, Stop, Svg } from "react-native-svg";

export const fontSize = {
    tiny: 13,
    small: 20,
    medium: 22,
    large: 30
}

export const defaultFont = 'SplineSansMono-Light';
export const defaultFontItalic = 'SplineSansMono-LightItalic';

export const styleSheet = StyleSheet.create({
    input: {

    }
});

export const CustomLinearGradient = ({children, style, from, to}: {children: any, style:any, from: ColorValue, to: ColorValue}) => {
    return (
        <View style={style}>
            <Svg height="100%" width="100%" style={ StyleSheet.absoluteFillObject }>
                <Defs>
                    <LinearGradient id="grad" x1="0%" y1="0%" x2="0%" y2="100%">
                        <Stop offset="0" stopColor={ from }/>
                        <Stop offset="1" stopColor={ to }/>
                    </LinearGradient>
                </Defs>
                <Rect width="100%" height="100%" fill="url(#grad)"/>
            </Svg>
            {children}
        </View>
    );
};

export const colorScheme = {

    // --text: #ebe9fc;
    // --background: #010104;
    // --primary: #3a31d8;
    // --secondary: #020024;
    // --accent: #0600c2;

    // love hashira
    // --text: #f6dffa;
    // --background: #08020a;
    // --primary: #d586eb;
    // --secondary: #924118;
    // --accent: #dfc639;

    // purple
    // --text: #f1e5f6;
    // --background: #0b040d;
    // --primary: #ca86e5;
    // --secondary: #6b168d;
    // --accent: #b739e9;

    // aqua
    // --text: #e8f6f5;
    // --background: #030909;
    // --primary: #95dbd4;
    // --secondary: #2a837a;
    // --accent: #40c7ba;

    // purple + orange
    // --text: #efe8fa;
    // --background: #090413;
    // --primary: #a285e6;
    // --secondary: #8e1d30;
    // --accent: #da8348;

    // blue
    // --text: #f0f0f4;
    // --background: #05060b;
    // --primary: #99a2e2;
    // --secondary: #162799;
    // --accent: #1633f7;

    // blue + purple
    // --text: #e2f4fb;
    // --background: #020c10;
    // --primary: #7bc6eb;
    // --secondary: #59179a;
    // --accent: #c830df;

    background: '#000',
    secondary: '#0d131f',
    tertiary_alternative: '#8eaae8',
    tertiary: '#942dd4', // text
    primary: '#fff',
    link: '#7dcfff', // hyperlink
    border: '#222936'

    // background: '#000',
    // secondary: '#222',
    // primary: '#fff', // text
    // link: '#7dcfff', // hyperlink
    // border: '#1a1a1a'

    // spline sans mono
    // noto sans mono

    // inter
    // Teachers
    // Readex Pro
}
