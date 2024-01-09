// The original content is temporarily commented out to allow generating a self-contained demo - feel free to uncomment later.

// import 'package:flutter/material.dart';
// import 'package:flutter/cupertino.dart';
// import 'package:flutter/services.dart';
// import 'package:flutter_platform_widgets/flutter_platform_widgets.dart';
// import 'package:provider/provider.dart';
// import 'home.dart';
// import 'state.dart';
// 
// void main() {
//   runApp(ChangeNotifierProvider(
//     create: (context) => FARState(),
//     child: const FARApp(),
//   ));
//   SystemChrome.setPreferredOrientations([
//     DeviceOrientation.portraitUp,
//     DeviceOrientation.portraitDown,
//   ]);
// }
// 
// class FARApp extends StatelessWidget {
//   const FARApp({super.key});
// 
//   @override
//   Widget build(BuildContext context) {
//     return PlatformProvider(
//       builder: (context) => PlatformTheme(
//         builder: (context) => const PlatformApp(
//           localizationsDelegates: <LocalizationsDelegate<dynamic>>[
//             DefaultMaterialLocalizations.delegate,
//             DefaultWidgetsLocalizations.delegate,
//             DefaultCupertinoLocalizations.delegate,
//           ],
//           title: 'First Aid Robot',
//           home: FARHomePage(),
//         ),
//       ),
//     );
//   }
// }
// 

import 'package:flutter/material.dart';
import 'package:first_aid_robot/src/rust/api/simple.dart';
import 'package:first_aid_robot/src/rust/frb_generated.dart';

Future<void> main() async {
  await RustLib.init();
  runApp(const MyApp());
}

class MyApp extends StatelessWidget {
  const MyApp({super.key});

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      home: Scaffold(
        appBar: AppBar(title: const Text('flutter_rust_bridge quickstart')),
        body: Center(
          child: Text(
              'Action: Call Rust `greet("Tom")`\nResult: `${greet(name: "Tom")}`'),
        ),
      ),
    );
  }
}
