import 'package:first_aid_robot/bridge_generated.dart';
// import 'package:first_aid_robot/bridge_definitions.dart';
import 'package:flutter_rust_bridge/flutter_rust_bridge.dart';

NativeImpl initializeExternalLibrary(String path) {
  return NativeImpl(loadLibForFlutter(path));
}
