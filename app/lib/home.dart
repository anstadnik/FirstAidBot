import 'package:flutter/material.dart';
import 'package:provider/provider.dart';
import 'package:flutter_platform_widgets/flutter_platform_widgets.dart';
import 'state.dart';
import 'state_widget.dart';

class FARHomePage extends StatelessWidget {
  const FARHomePage({Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return PlatformScaffold(
      iosContentPadding: true,
      appBar: PlatformAppBar(title: const Text('First Aid Robot')),
      body: const FABFutureBuilder(),
      bottomNavBar: _buildNavBar(context),
    );
  }

  PlatformNavBar _buildNavBar(BuildContext context) {
    return PlatformNavBar(
      items: [
        BottomNavigationBarItem(
            icon: Icon(context.platformIcons.book), label: 'Bot'),
        BottomNavigationBarItem(
            icon: Icon(context.platformIcons.info), label: 'Info'),
      ],
    );
  }
}

class FABFutureBuilder extends StatelessWidget {
  const FABFutureBuilder({Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    var farState = Provider.of<FARState>(context, listen: false);
    return Column(
      mainAxisAlignment: MainAxisAlignment.spaceBetween,
      children: [
        const Expanded(child: StateConsumer()),
        Padding(
          padding: const EdgeInsets.all(16.0),
          child: Row(mainAxisAlignment: MainAxisAlignment.spaceBetween, children: [
            _buildButtonCallback("Back", () => farState.back()),
            _buildButtonCallback("Home", () => farState.home()),
            _buildButtonCallback("Refresh", () => farState.refresh())
          ]),
        ),
      ],
    );
  }

  Widget _buildButtonCallback(String text, VoidCallback cbk) =>
      PlatformElevatedButton(
        color: Colors.orange,
        onPressed: cbk,
        child: Text(text),
      );
}

class StateConsumer extends StatelessWidget {
  const StateConsumer({
    super.key,
  });

  @override
  Widget build(BuildContext context) {
    return Consumer<FARState>(builder: (context, state, child) {
      if (state.updating) {
        return const Center(child: CircularProgressIndicator());
      } else if (state.faMLFS == null) {
        showPlatformDialog(
            context: context,
            builder: (context) => PlatformAlertDialog(
                  title: const Text('Error'),
                  content: const Text('Failed to load data'),
                  actions: [
                    PlatformDialogAction(
                        child: const Text('OK'),
                        onPressed: () => Navigator.pop(context))
                  ],
                ));
        return const Column(
          mainAxisAlignment: MainAxisAlignment.center,
          children: [
            Icon(Icons.broken_image),
            Text("Couldn't load data, please try again later"),
          ],
        );
      } else {
        return StateWidget(faState: state.faState!);
      }
    });
  }
}
