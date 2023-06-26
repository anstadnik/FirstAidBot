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
  Widget build(BuildContext context) => Column(
        // mainAxisSize: MainAxisSize.max,
        // mainAxisAlignment: MainAxisAlignment.spaceBetween,
        children: [
          SingleChildScrollView(
            child: Text(
              faState.message,
              style: const TextStyle(fontSize: 20),
              // textAlign: TextAlign.center,
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
      child: GridView.count(
        crossAxisCount: 2,
        crossAxisSpacing: 16.0,
        mainAxisSpacing: 16.0,
        shrinkWrap: true,
        childAspectRatio: 5,
        children:
            nextStates.map((nextState) => _buildButton(nextState)).toList(),
      ));

  Widget _buildButton(String nextState) => Consumer<FARState>(
      builder: (context, state, child) => PlatformElevatedButton(
            onPressed: () => state.transition(nextState),
            child: Text(nextState),
          ));
}
