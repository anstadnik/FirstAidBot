// import 'package:first_aid_robot/bridge_generated.dart';
import 'package:first_aid_robot/bridge_definitions.dart';
import 'package:flutter_test/flutter_test.dart';

import 'ffi.test.dart';

Future<void> main() async {
  final api = initializeExternalLibrary('../target/debug/libnative.dylib');

  test('User save/load', () async {
    MultilangFs faMLFS = await api.getData();
    RwLockFaContext faCTX = api.getContext();
    FAState? faState = await api.getFs(mlfs: faMLFS, ctx: faCTX);
    expect(faState, isNotNull);
  });
}
