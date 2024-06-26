import 'package:flutter/material.dart';
import 'package:flutter/cupertino.dart';
import 'package:flutter/services.dart';
import 'package:flutter_platform_widgets/flutter_platform_widgets.dart';
import 'package:provider/provider.dart';
import 'package:app/src/rust/frb_generated.dart';
import 'home.dart';
import 'state.dart';

Future<void> main() async {
  await RustLib.init();
  runApp(ChangeNotifierProvider(
    create: (context) => FARState(),
    child: const FARApp(),
  ));
  SystemChrome.setPreferredOrientations([
    DeviceOrientation.portraitUp,
    DeviceOrientation.portraitDown,
  ]);
}

class FARApp extends StatelessWidget {
  const FARApp({super.key});

  @override
  Widget build(BuildContext context) {
    return PlatformProvider(
      builder: (context) => PlatformTheme(
        builder: (context) => const PlatformApp(
          localizationsDelegates: <LocalizationsDelegate<dynamic>>[
            DefaultMaterialLocalizations.delegate,
            DefaultWidgetsLocalizations.delegate,
            DefaultCupertinoLocalizations.delegate,
          ],
          title: 'First Aid Robot',
          home: FARHomePage(),
        ),
      ),
    );
  }
}
