import 'state.dart';
// import 'ffi.dart' if (dart.library.html) 'ffi_web.dart';
import 'package:flutter/material.dart';
import 'package:provider/provider.dart';
import 'package:flutter_platform_widgets/flutter_platform_widgets.dart';
import 'package:app/src/rust/api/fa_api.dart';

class StateWidget extends StatelessWidget {
  const StateWidget({
    super.key,
    required this.fs,
  });

  final Fs_ fs;

  @override
  Widget build(BuildContext context) {
    ScrollController scrollController = ScrollController();
    return Column(
      // mainAxisSize: MainAxisSize.max,
      // mainAxisAlignment: MainAxisAlignment.spaceBetween,
      children: [
        Padding(
          padding: const EdgeInsets.all(00.0),
          child: Text(
            fs.message,
            style: const TextStyle(fontSize: 20),
            // textAlign: TextAlign.center,
          ),
        ),
        Padding(
          padding: const EdgeInsets.only(left: 16.0, right: 16.0),
          child: ButtonGrid(nextStates: fs.nextStates),
        ),
      ],
    );
  }
}

class ButtonGrid extends StatelessWidget {
  final List<String> nextStates;

  const ButtonGrid({required this.nextStates, super.key});

  @override
  Widget build(BuildContext context) => GridView.count(
        crossAxisCount: 2,
        crossAxisSpacing: 16.0,
        mainAxisSpacing: 16.0,
        shrinkWrap: true,
        childAspectRatio: 3.5,
        children:
            nextStates.map((nextState) => _buildButton(nextState)).toList(),
      );

  Widget _buildButton(String nextState) => Consumer<FARState>(
      builder: (context, state, child) => PlatformElevatedButton(
            onPressed: () => state.transition(nextState),
            child: Text(nextState,
                textAlign: TextAlign.center,
                style: const TextStyle(fontSize: 16)),
          ));
}
