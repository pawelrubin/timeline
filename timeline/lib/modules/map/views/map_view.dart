import 'package:flutter/material.dart';
import 'package:google_maps_flutter/google_maps_flutter.dart';
import 'package:provider/provider.dart';
import 'package:timeline/modules/core/services/api_service.dart';
import 'package:timeline/modules/core/widgets/layout.dart';

class MapView extends StatefulWidget {
  const MapView({Key? key}) : super(key: key);

  @override
  State<StatefulWidget> createState() => _MapViewState();
}

class _MapViewState extends State<MapView> {
  final CameraPosition _initial = const CameraPosition(target: LatLng(0, 0));
  final Set<Polyline> _polylines = {};
  final Set<Marker> _markers = {};

  _refreshData(BuildContext context) async {
    Provider.of<ApiService>(context, listen: false)
        .fetchData()
        .then((locationData) {
      locationData.sort((a, b) => a.timestamp.isAfter(b.timestamp) ? 1 : -1);

      if (locationData.length >= 2) {
        var startMarker = Marker(
            markerId: const MarkerId('start'),
            position: LatLng(
                locationData.first.latitude, locationData.first.longitude));
        var endMarker = Marker(
            markerId: const MarkerId('end'),
            position: LatLng(
                locationData.last.latitude, locationData.last.longitude));

        var polyline = Polyline(
            polylineId: const PolylineId('line'),
            points: locationData
                .map((e) => LatLng(e.latitude, e.longitude))
                .toList());

        setState(() {
          _markers.addAll({startMarker, endMarker});
          _polylines.add(polyline);
        });
      }
    }).catchError((e) {
      print(e);
      ScaffoldMessenger.of(context)
          .showSnackBar(const SnackBar(content: Text('Failed to load data')));
    });
  }

  @override
  void initState() {
    super.initState();

    _refreshData(context);
  }

  @override
  Widget build(BuildContext context) {
    return Layout(
      body: Center(
          child: GoogleMap(
        initialCameraPosition: _initial,
        markers: _markers,
        polylines: _polylines,
      )),
    );
  }
}
