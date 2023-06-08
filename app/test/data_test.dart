// import 'package:first_aid_robot/bridge_generated.dart';
import 'package:first_aid_robot/bridge_definitions.dart';
import 'package:flutter_test/flutter_test.dart';

import 'ffi.test.dart';

Future<void> main() async {
  final api = initializeExternalLibrary('../target/debug/libnative.dylib');

  testFs(MultilangFs faMLFS, RwLockFaContext faCTX, FAState? faState) async {
    expect(faState, isNotNull, reason: await api.getPath(ctx: faCTX));
    print("0. ${faState?.message}");
    print("0. ${faState?.nextStates}");
    for (String next in faState!.nextStates) {
      print("1. ${await api.getPath(ctx: faCTX)}");
      api.transition(ctx: faCTX, text: next);
      print("2. ${await api.getPath(ctx: faCTX)}");
      FAState? state = await api.getFs(mlfs: faMLFS, ctx: faCTX);
      testFs(faMLFS, faCTX, state);
      print("3. ${await api.getPath(ctx: faCTX)}");
      api.back(ctx: faCTX);
      print("4. ${await api.getPath(ctx: faCTX)}");
    }
  }

  test('User save/load', () async {
    MultilangFs faMLFS = await api.getData();
    RwLockFaContext faCTX = api.getContext();
    FAState? faState = await api.getFs(mlfs: faMLFS, ctx: faCTX);
    await testFs(faMLFS, faCTX, faState);
  });
}
