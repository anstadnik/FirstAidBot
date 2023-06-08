import 'package:flutter/material.dart';
import 'package:provider/provider.dart';
import 'ffi.dart' if (dart.library.html) 'ffi_web.dart';
import 'package:flutter_platform_widgets/flutter_platform_widgets.dart';
import 'state.dart';

class FARHomePage extends StatelessWidget {
  const FARHomePage({Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return PlatformScaffold(
      iosContentPadding: true,
      appBar: PlatformAppBar(title: const Text('First Aid Robot')),
      body: const MyFutureBuilder(),
      // bottomNavBar: _buildNavBar(context),
    );
  }

  PlatformNavBar _buildNavBar(BuildContext context) {
    return PlatformNavBar(
      items: [
        BottomNavigationBarItem(
            icon: Icon(context.platformIcons.back), label: 'Back'),
        BottomNavigationBarItem(
            icon: Icon(context.platformIcons.home), label: 'Home'),
      ],
    );
  }
}

class MyFutureBuilder extends StatelessWidget {
  const MyFutureBuilder({Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return Consumer<FARState>(builder: (context, state, child) {
      if (state.faState == null) {
        return const Center(child: CircularProgressIndicator());
      } else {
        return _buildContent(state.faState!);
      }
    });
  }

  Widget _buildContent(FAState faState) => Column(
        mainAxisAlignment: MainAxisAlignment.spaceBetween,
        children: [
          Padding(
            padding: const EdgeInsets.all(16.0),
            child: Text(
              faState.message,
              style: const TextStyle(fontSize: 20),
              textAlign: TextAlign.center,
            ),
          ),
          ButtonGrid(nextStates: faState.nextStates),
        ],
      );
}

class ButtonGrid extends StatelessWidget {
  final List<String> nextStates;

  const ButtonGrid({required this.nextStates, Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context) => Padding(
      padding: const EdgeInsets.symmetric(horizontal: 16.0),
      child: Column(mainAxisAlignment: MainAxisAlignment.start, children: [
        GridView.count(
          crossAxisCount: 2,
          crossAxisSpacing: 16.0,
          mainAxisSpacing: 16.0,
          shrinkWrap: true,
          childAspectRatio: 5,
          children:
              nextStates.map((nextState) => _buildButton(nextState)).toList(),
        ),
        const SizedBox(height: 16.0),
        Consumer<FARState>(
            builder: (context, state, child) => Row(
                    mainAxisAlignment: MainAxisAlignment.spaceEvenly,
                    children: [
                      _buildButtonCallback("Back", () => state.back()),
                      _buildButtonCallback("Home", () => state.home()),
                      _buildButtonCallback("Refresh", () => state.refresh())
                    ])),
        const SizedBox(height: 16.0),
      ]));

  PlatformElevatedButton _buildButtonCallback(String text, VoidCallback cbk) =>
      PlatformElevatedButton(
        color: Colors.orange,
        onPressed: cbk,
        child: Text(text),
      );

  Widget _buildButton(String nextState) => Consumer<FARState>(
      builder: (context, state, child) => PlatformElevatedButton(
            onPressed: () => state.transition(nextState),
            child: Text(nextState),
          ));
}
