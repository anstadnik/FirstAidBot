import 'package:flutter/material.dart';
import 'package:flutter/cupertino.dart';
import 'package:flutter_platform_widgets/flutter_platform_widgets.dart';
import 'package:provider/provider.dart';
import 'home.dart';
import 'state.dart';

void main() {
  runApp(ChangeNotifierProvider(
    create: (context) => FARState(),
    child: const FARApp(),
  ));
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
