import 'dart:async';

import 'package:flutter/material.dart';
import 'package:google_maps_flutter/google_maps_flutter.dart';
import 'package:provider/provider.dart';
import 'package:timeline/modules/core/model/location_entry.dart';
import 'package:timeline/modules/core/services/api_service.dart';
import 'package:timeline/modules/core/widgets/layout.dart';
import 'package:flutter_datetime_picker/flutter_datetime_picker.dart';

class MapView extends StatefulWidget {
  const MapView({Key? key}) : super(key: key);

  @override
  State<StatefulWidget> createState() => _MapViewState();

  static LatLngBounds getLatLngBounds(List<LocationEntry> list) {
    double x0 = list.first.latitude, x1 = list.first.latitude;
    double y0 = list.first.longitude, y1 = list.first.longitude;

    for (final latLng in list) {
      if (latLng.latitude > x1) x1 = latLng.latitude;
      if (latLng.latitude < x0) x0 = latLng.latitude;
      if (latLng.longitude > y1) y1 = latLng.longitude;
      if (latLng.longitude < y0) y0 = latLng.longitude;
    }

    return LatLngBounds(northeast: LatLng(x1, y1), southwest: LatLng(x0, y0));
  }
}

class _MapViewState extends State<MapView> {
  final Completer<GoogleMapController> _mapController = Completer();
  final Set<Polyline> _polylines = {};
  final Set<Marker> _markers = {};

  DateTime _date = DateTime.now();

  _refreshData(BuildContext context) async {
    Provider.of<ApiService>(context, listen: false)
        .fetchData(_date)
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
            color: Colors.blue,
            polylineId: const PolylineId('line'),
            points: locationData
                .map((e) => LatLng(e.latitude, e.longitude))
                .toList());

        _mapController.future.then((controller) => controller.moveCamera(
            CameraUpdate.newLatLngBounds(
                MapView.getLatLngBounds(locationData), 50)));

        setState(() {
          _polylines.clear();
          _markers.clear();
          _markers.addAll({startMarker, endMarker});
          _polylines.add(polyline);
        });
      } else {
        setState(() {
          _markers.clear();
          _polylines.clear();
        });
      }
    }).catchError((e) {
      ScaffoldMessenger.of(context)
          .showSnackBar(const SnackBar(content: Text('Failed to load data')));
    });
  }

  _setDate(DateTime? date) {
    if (date != null) {
      setState(() {
        _date = date;
      });
      _refreshData(context);
    }
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
          child: Column(
        children: [
          TextButton(
              onPressed: () => DatePicker.showDatePicker(context,
                  onConfirm: _setDate, currentTime: _date),
              child: const Text('Choose date')),
          Expanded(
              child: GoogleMap(
            onMapCreated: (GoogleMapController controller) {
              _mapController.complete(controller);
            },
            initialCameraPosition:
                const CameraPosition(target: LatLng(0, 0), zoom: 0),
            markers: _markers,
            polylines: _polylines,
          ))
        ],
      )),
    );
  }
}
