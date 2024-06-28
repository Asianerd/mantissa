import { Button, Image, Pressable, ScrollView, StyleSheet, Text, TextInput, View } from "react-native";
import { screenSize } from "../App";
import React from "react";
import { CustomLinearGradient, colorScheme, defaultFont, defaultFontItalic, fontSize } from "../constants/style";
import { Defs, LinearGradient, Rect, Stop, Svg } from "react-native-svg";

function Login(): React.JSX.Element {
    const loginStyles = StyleSheet.create({
        input: {
            width:0.7 * screenSize.width,

            borderStyle: "solid",
            borderColor: colorScheme.border,
            borderBottomWidth: 1,

            paddingTop:20,
            paddingBottom:5,

            fontSize: fontSize.small,
            fontFamily: defaultFont
        },
        button: {
            // backgroundColor: `linear-gradient(90deg, ${colorScheme.tertiary} 50%, ${colorScheme.tertiary_alternative} 100%)`,
            backgroundColor: colorScheme.secondary,
            padding:8,
            paddingHorizontal:15,
            borderRadius:10,

            overflow:'hidden'
        }
    });

    const [state, onStateChange] = React.useState('login');

    const [username, onUsernameChange] = React.useState('');
    const [password, onPasswordChange] = React.useState('');
    const [confirmPassword, onConfirmPasswordChange] = React.useState('');

    return <View style={{
        backgroundColor: colorScheme.background,
        flex: 1,
        display: 'flex',
        justifyContent: 'center',
        alignItems: 'center'
        }}>
            <Image source={require('../assets/sns/github.png')} style={{
                borderRadius: 1000,
                height:'20%',
                aspectRatio: 1,
                marginBottom:20
            }} />
            <TextInput style={loginStyles.input}
                placeholder="username"
                onChangeText={onUsernameChange}
            />
            <TextInput style={loginStyles.input}
                placeholder="password"
                onChangeText={onPasswordChange}
                secureTextEntry={true}
            />
            <TextInput style={[ loginStyles.input,
                    {
                        display:(state == 'login' ? 'none' : 'flex')
                    }
                ]}
                placeholder="confirm password"
                onChangeText={onConfirmPasswordChange}
                secureTextEntry={true}
            />
            <View style={{
                width: 0.7 * screenSize.width,
                display: 'flex',
                alignItems: 'center',
                flexDirection: 'column',

                marginTop:50,
            }}>
                <Pressable style={loginStyles.button} onPress={() => { console.log(`login by ${username} with ${password}`); }}>
                    <Text style={{ color:colorScheme.primary, fontSize: fontSize.medium, fontFamily:defaultFont }}>
                        {
                            state == 'login' ? 'log in' : 'sign up'
                        }
                    </Text>
                </Pressable>
                <Pressable style={{ marginTop:10 }} onPress={() => { onStateChange(state == 'login' ? 'signup' : 'login') }}>
                    <Text style={{ color:colorScheme.link, fontSize: fontSize.tiny, fontFamily: defaultFontItalic }}>
                        {
                            state == 'login' ? 'dont have an account? sign up' : 'have an account? log in'
                        }
                    </Text>
                </Pressable>
            </View>
    </View>
}

export default Login;
