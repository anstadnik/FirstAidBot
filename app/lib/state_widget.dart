import 'state.dart';
import 'ffi.dart' if (dart.library.html) 'ffi_web.dart';
import 'package:flutter/material.dart';
import 'package:provider/provider.dart';
import 'package:flutter_platform_widgets/flutter_platform_widgets.dart';

class StateWidget extends StatelessWidget {
  const StateWidget({
    super.key,
    required this.faState,
  });

  final FAState faState;

  @override
  Widget build(BuildContext context) {
    ScrollController scrollController = ScrollController();
    return Column(
      // mainAxisSize: MainAxisSize.max,
      // mainAxisAlignment: MainAxisAlignment.spaceBetween,
      children: [
        Expanded(
          child: Padding(
            padding: const EdgeInsets.all(2.0),
            child: Scrollbar(
              thumbVisibility: true,
              trackVisibility: true,
              controller: scrollController,
              child: SingleChildScrollView(
                controller: scrollController,
                child: Padding(
                  padding: const EdgeInsets.all(16.0),
                  child: Text(
                    faState.message,
                    // style: const TextStyle(fontSize: 12),
                    // textAlign: TextAlign.center,
                  ),
                ),
              ),
            ),
          ),
        ),
        Padding(
          padding: const EdgeInsets.all(16.0),
          child: ButtonGrid(nextStates: faState.nextStates),
        ),
      ],
    );
  }
}

class ButtonGrid extends StatelessWidget {
  final List<String> nextStates;

  const ButtonGrid({required this.nextStates, Key? key}) : super(key: key);

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
            child: Text(nextState, textAlign: TextAlign.center),
          ));
}
